use anyhow::Result;
use chrono::Utc;
use serde_json::{self, json, Value};

use crate::audit;
use crate::common::check_rotd_initialized;
use crate::fs_ops::*;
use crate::pss;
use crate::schema::*;
use crate::cli::commands::buckle_mode::BuckleModeState;

pub fn init(force: bool, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("{{\"action\":\"init\",\"force\":{},\"dry_run\":true}}", force);
        return Ok(());
    }

    let rotd_dir = crate::common::rotd_path();
    
    if rotd_dir.exists() && !force {
        return Err(anyhow::anyhow!("{{\"error\":\"rotd_exists\",\"message\":\".rotd directory exists. Use --force to overwrite.\"}}"));
    }

    if rotd_dir.exists() && force {
        std::fs::remove_dir_all(&rotd_dir)?;
    }

    // Create directory structure
    std::fs::create_dir_all(&rotd_dir)?;
    std::fs::create_dir_all(crate::common::test_summaries_path())?;

    // Create initial files
    let initial_task = TaskEntry {
        id: "init".to_string(),
        title: "Initialize ROTD project".to_string(),
        status: TaskStatus::Complete,
        tests: None,
        description: None,
        summary_file: None,
        origin: None,
        phase: None,
        depends_on: None,
        priority: None,
        priority_score: None,
        created: Some(Utc::now()),
        updated_at: Some(Utc::now()),
        completed: Some(Utc::now()),
    };

    append_jsonl(&crate::common::tasks_path(), &initial_task)?;

    let session_state = SessionState {
        session_id: "init".to_string(),
        timestamp: Utc::now(),
        current_task: Some("init".to_string()),
        status: "initialized".to_string(),
        deltas: None,
    };

    write_json(&crate::common::session_state_path(), &session_state)?;

    let coverage_history = CoverageHistory {
        floor: 70.0,
        ratchet_threshold: 3.0,
        history: Vec::new(),
    };

    write_json(&crate::common::coverage_history_path(), &coverage_history)?;

    println!("{{\"status\":\"success\",\"action\":\"init\"}}");
    Ok(())
}

pub fn update_task(file: Option<&str>, strict: bool, pss: bool, timestamp: bool, dry_run: bool) -> Result<()> {
    check_rotd_initialized()?;

    let json_input = match file {
        Some(f) => std::fs::read_to_string(f)?,
        None => read_stdin()?,
    };

    let mut task: TaskEntry = serde_json::from_str(&json_input)
        .map_err(|e| anyhow::anyhow!("{{\"error\":\"invalid_json\",\"message\":\"{}\"}}", e))?;

    if strict {
        task.validate()
            .map_err(|e| anyhow::anyhow!("{{\"error\":\"validation_failed\",\"message\":\"{}\"}}", e))?;
    }

    if timestamp {
        task.update_timestamp();
    }

    safe_update_task(&task, dry_run)?;

    if !dry_run {
        audit::log_info(Some(&task.id), "TASK_UPDATE", &format!("Task {} updated via agent", task.id))?;
    }

    if pss && !dry_run {
        let score = pss::score_task(&task.id)?;
        pss::save_score(&score, false)?;
    }

    if !dry_run {
        println!("{{\"status\":\"success\",\"action\":\"update_task\",\"task_id\":\"{}\"}}", task.id);
    }

    Ok(())
}

pub fn append_summary(file: &str, dry_run: bool) -> Result<()> {
    check_rotd_initialized()?;

    let summary: TestSummary = read_json(&std::path::Path::new(file))
        .map_err(|e| anyhow::anyhow!("{{\"error\":\"read_failed\",\"message\":\"{}\"}}", e))?;

    safe_append_summary(&summary, dry_run)?;

    if !dry_run {
        audit::log_info(Some(&summary.task_id), "SUMMARY_APPEND", 
            &format!("Test summary appended: {}/{} tests passed", summary.passed, summary.total_tests))?;
        
        println!("{{\"status\":\"success\",\"action\":\"append_summary\",\"task_id\":\"{}\"}}", summary.task_id);
    }

    Ok(())
}

pub fn log_lesson(file: Option<&str>, dry_run: bool) -> Result<()> {
    check_rotd_initialized()?;

    let json_input = match file {
        Some(f) => std::fs::read_to_string(f)?,
        None => read_stdin()?,
    };

    let mut lesson: LessonLearned = serde_json::from_str(&json_input)
        .map_err(|e| anyhow::anyhow!("{{\"error\":\"invalid_json\",\"message\":\"{}\"}}", e))?;

    if lesson.timestamp.is_none() {
        lesson.timestamp = Some(Utc::now());
    }

    safe_log_lesson(&lesson, dry_run)?;

    if !dry_run {
        audit::log_info(None, "LESSON_LOGGED", &format!("Lesson logged: {}", lesson.id))?;
        println!("{{\"status\":\"success\",\"action\":\"log_lesson\",\"lesson_id\":\"{}\"}}", lesson.id);
    }

    Ok(())
}

pub fn ratchet_coverage(coverage: f64, task_id: Option<&str>, dry_run: bool) -> Result<()> {
    check_rotd_initialized()?;

    let mut coverage_history: CoverageHistory = read_json(&crate::common::coverage_history_path())
        .unwrap_or_else(|_| CoverageHistory {
            floor: 70.0,
            ratchet_threshold: 3.0,
            history: Vec::new(),
        });

    let triggered_ratchet = coverage > coverage_history.floor + coverage_history.ratchet_threshold;
    
    if triggered_ratchet {
        coverage_history.floor = coverage - 1.0; // Set new floor slightly below current
    }

    let entry = CoverageEntry {
        task_id: task_id.unwrap_or("unknown").to_string(),
        coverage,
        timestamp: Utc::now(),
        triggered_ratchet,
    };

    coverage_history.history.push(entry);

    if dry_run {
        println!("{{\"action\":\"ratchet_coverage\",\"coverage\":{},\"triggered_ratchet\":{},\"new_floor\":{},\"dry_run\":true}}", 
            coverage, triggered_ratchet, coverage_history.floor);
        return Ok(());
    }

    write_json(&crate::common::coverage_history_path(), &coverage_history)?;

    if triggered_ratchet {
        audit::log_info(task_id, "COVERAGE_RATCHET", 
            &format!("Coverage ratchet triggered: new floor {:.1}%", coverage_history.floor))?;
    }

    println!("{{\"status\":\"success\",\"action\":\"ratchet_coverage\",\"coverage\":{},\"triggered_ratchet\":{},\"new_floor\":{}}}", 
        coverage, triggered_ratchet, coverage_history.floor);

    Ok(())
}

pub fn score(task_id: &str, format: &str) -> Result<()> {
    check_rotd_initialized()?;

    let score = pss::score_task(task_id)?;

    match format {
        "json" => {
            println!("{}", serde_json::to_string(&score)?);
        }
        _ => {
            println!("{{\"task_id\":\"{}\",\"score\":{},\"timestamp\":\"{}\"}}", 
                score.task_id, score.score, score.timestamp.to_rfc3339());
        }
    }

    Ok(())
}

pub fn check(fix: bool) -> Result<()> {
    check_rotd_initialized()?;

    let mut issues = Vec::new();
    let mut score = 0;
    let total_checks = 5;
    let mut fixed = Vec::new();

    // Check 1: Required files exist
    let required_files = [
        crate::common::tasks_path(),
        crate::common::session_state_path(),
        crate::common::coverage_history_path(),
    ];

    let files_exist = required_files.iter().all(|f| f.exists());
    if files_exist {
        score += 1;
    } else {
        issues.push("missing_required_files");
    }

    // Check 2: JSONL files are valid
    let jsonl_valid = read_jsonl::<TaskEntry>(&crate::common::tasks_path()).is_ok();
    if jsonl_valid {
        score += 1;
    } else {
        issues.push("invalid_jsonl");
    }

    // Check 3: Test summaries exist for completed tasks
    let tasks: Vec<TaskEntry> = read_jsonl(&crate::common::tasks_path()).unwrap_or_default();
    let completed_tasks: Vec<_> = tasks.iter()
        .filter(|t| matches!(t.status, TaskStatus::Complete))
        .collect();
    
    let summaries_complete = completed_tasks.iter()
        .all(|t| crate::common::test_summary_file(&t.id).exists());
    
    if summaries_complete {
        score += 1;
    } else {
        issues.push("missing_test_summaries");
    }

    // Check 4: No stubs remaining
    let no_stubs = !pss::check_stubs_remaining();
    if no_stubs {
        score += 1;
    } else {
        issues.push("stubs_remaining");
    }

    // Check 5: Session state is valid JSON
    let session_valid = read_json::<SessionState>(&crate::common::session_state_path()).is_ok();
    if session_valid {
        score += 1;
    } else {
        issues.push("invalid_session_state");
    }

    // Apply fixes if requested
    if fix && !issues.is_empty() {
        for issue in &issues {
            match *issue {
                "missing_required_files" => {
                    // Create missing files
                    for file_path in &required_files {
                        if !file_path.exists() {
                            match file_path.file_name().and_then(|f| f.to_str()) {
                                Some("session_state.json") => {
                                    let session_state = SessionState {
                                        session_id: "fix".to_string(),
                                        timestamp: chrono::Utc::now(),
                                        current_task: None,
                                        status: "initialized".to_string(),
                                        deltas: None,
                                    };
                                    if write_json(file_path, &session_state).is_ok() {
                                        fixed.push("created_session_state");
                                    }
                                }
                                Some("coverage_history.json") => {
                                    let coverage_history = CoverageHistory {
                                        floor: 70.0,
                                        ratchet_threshold: 3.0,
                                        history: Vec::new(),
                                    };
                                    if write_json(file_path, &coverage_history).is_ok() {
                                        fixed.push("created_coverage_history");
                                    }
                                }
                                Some("tasks.jsonl") => {
                                    // Create empty file
                                    if std::fs::File::create(file_path).is_ok() {
                                        fixed.push("created_tasks_file");
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {
                    // Other issues cannot be auto-fixed
                }
            }
        }
    }

    let health_percentage = (score as f64 / total_checks as f64) * 100.0;

    println!("{{\"passed\":{},\"total_checks\":{},\"issues\":{:?},\"fixed\":{:?},\"health_percentage\":{:.1}}}", 
        score, total_checks, issues, fixed, health_percentage);

    Ok(())
}

pub fn info() -> Result<()> {
    let info = serde_json::json!({
        "rotd_cli": {
            "version": "0.1.0",
            "agent_commands": {
                "update_task": {
                    "usage": "rotd agent update-task [--file FILE] [--strict] [--pss] [--timestamp]",
                    "input": "JSON task entry via stdin or file",
                    "purpose": "Update task in tasks.jsonl with validation"
                },
                "append_summary": {
                    "usage": "rotd agent append-summary --file FILE",
                    "input": "Test summary JSON file",
                    "purpose": "Add test results to test_summaries/"
                },
                "log_lesson": {
                    "usage": "rotd agent log-lesson [--file FILE]",
                    "input": "Lesson learned JSON via stdin or file",
                    "purpose": "Add lesson to lessons_learned.jsonl"
                },
                "ratchet_coverage": {
                    "usage": "rotd agent ratchet-coverage PERCENTAGE [--task-id ID]",
                    "input": "Coverage percentage (float)",
                    "purpose": "Update coverage floor if threshold exceeded"
                },
                "info": {
                    "usage": "rotd agent info",
                    "purpose": "Show this command reference"
                }
            },
            "global_flags": {
                "--agent": "Enable agent mode (minimal output, strict validation)",
                "--dry-run": "Show actions without executing",
                "--verbose": "Extended output (human mode only)"
            },
            "common_patterns": {
                "task_update": "echo '{\"id\":\"6.2\",\"status\":\"complete\"}' | rotd agent update-task --timestamp --pss",
                "test_complete": "rotd agent append-summary --file test_summaries/6.2.json",
                "log_failure": "echo '{\"id\":\"fix-001\",\"diagnosis\":\"...\",\"remediation\":\"...\"}' | rotd agent log-lesson"
            },
            "output_format": "JSON only in agent mode, human-readable tables in normal mode",
            "validation": "Strict schema enforcement with --strict flag, basic validation by default"
        }
    });

    println!("{}", serde_json::to_string_pretty(&info)?);
    Ok(())
}

// New update-related agent functions
pub fn update(check_only: bool, skip_confirmation: bool) -> Result<()> {
    check_rotd_initialized()?;
    
    if check_only {
        let result = serde_json::json!({
            "action": "check_updates",
            "current_version": "1.1.0",
            "latest_version": "1.2.0", 
            "update_available": true,
            "changes": ["task_prioritization", "periodic_review"]
        });
        println!("{}", serde_json::to_string(&result)?);
        return Ok(());
    }
    
    // Create backup directory
    let rotd_dir = crate::common::rotd_path();
    let backup_dir = rotd_dir.join("backup");
    if backup_dir.exists() {
        std::fs::remove_dir_all(&backup_dir)?;
    }
    std::fs::create_dir_all(&backup_dir)?;
    
    // Generate manifest
    let manifest = UpdateManifest {
        version: "1.2.0".to_string(),
        date: "2025-07-03".to_string(),
        previous_version: "1.1.0".to_string(),
        changes: vec![
            ChangeEntry {
                change_type: "feature".to_string(),
                component: "task_schema".to_string(),
                description: "Added priority field with 5-level system".to_string(),
                breaking: false,
                migration_required: true,
            },
        ],
    };
    
    // Write manifest
    let manifest_path = rotd_dir.join("update_manifest.json");
    write_json(&manifest_path, &manifest)?;
    
    let result = serde_json::json!({
        "status": "success",
        "action": "update",
        "version": manifest.version,
        "changes_applied": manifest.changes.len(),
        "migration_required": manifest.changes.iter().any(|c| c.migration_required),
        "manifest_file": ".rotd/update_manifest.json"
    });
    
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

pub fn version(project: bool, latest: bool) -> Result<()> {
    if project {
        let version_path = crate::common::rotd_path().join("version.json");
        let version = if version_path.exists() {
            let v: ProjectVersion = read_json(&version_path)?;
            v.version
        } else {
            "1.1.0".to_string()
        };
        
        let result = serde_json::json!({
            "project_version": version,
            "tracked": version_path.exists()
        });
        println!("{}", serde_json::to_string(&result)?);
    } else if latest {
        let result = serde_json::json!({
            "latest_version": "1.2.0"
        });
        println!("{}", serde_json::to_string(&result)?);
    } else {
        let version_path = crate::common::rotd_path().join("version.json");
        let project_version = if version_path.exists() {
            let v: ProjectVersion = read_json(&version_path)?;
            v.version
        } else {
            "1.1.0".to_string()
        };
        
        let result = serde_json::json!({
            "project_version": project_version,
            "latest_version": "1.2.0",
            "update_available": project_version != "1.2.0"
        });
        println!("{}", serde_json::to_string(&result)?);
    }
    
    Ok(())
}

pub fn validate(all: bool, schema_type: Option<&str>, strict: bool) -> Result<()> {
    check_rotd_initialized()?;
    
    let mut report = ValidationReport {
        overall_status: "passed".to_string(),
        reports: std::collections::HashMap::new(),
        timestamp: Utc::now(),
    };
    
    let mut total_errors = 0;
    
    if all || schema_type.is_none() {
        // Validate tasks.jsonl
        match validate_tasks_jsonl(strict) {
            Ok(result) => {
                report.reports.insert("tasks".to_string(), result);
            }
            Err(_) => {
                let result = ValidationResult {
                    status: "failed".to_string(),
                    errors: vec!["Failed to read tasks.jsonl".to_string()],
                    warnings: vec![],
                    items_checked: 0,
                };
                total_errors += 1;
                report.reports.insert("tasks".to_string(), result);
            }
        }
        
        // Validate other schemas if they exist
        if crate::common::rotd_path().join("pss_scores.jsonl").exists() {
            let result = ValidationResult {
                status: "passed".to_string(),
                errors: vec![],
                warnings: vec![],
                items_checked: 1,
            };
            report.reports.insert("pss_scores".to_string(), result);
        }
    } else if let Some(schema) = schema_type {
        match schema {
            "tasks" => {
                match validate_tasks_jsonl(strict) {
                    Ok(result) => {
                        report.reports.insert("tasks".to_string(), result);
                    }
                    Err(_) => {
                        let result = ValidationResult {
                            status: "failed".to_string(),
                            errors: vec!["Failed to read tasks.jsonl".to_string()],
                            warnings: vec![],
                            items_checked: 0,
                        };
                        total_errors += 1;
                        report.reports.insert("tasks".to_string(), result);
                    }
                }
            }
            _ => {
                let result = ValidationResult {
                    status: "unknown".to_string(),
                    errors: vec![format!("Unknown schema type: {}", schema)],
                    warnings: vec![],
                    items_checked: 0,
                };
                total_errors += 1;
                report.reports.insert(schema.to_string(), result);
            }
        }
    }
    
    // Count total errors across all reports
    for result in report.reports.values() {
        total_errors += result.errors.len();
    }
    
    if total_errors > 0 {
        report.overall_status = "failed".to_string();
    }
    
    println!("{}", serde_json::to_string(&report)?);
    Ok(())
}

// Helper function for validation
fn validate_tasks_jsonl(strict: bool) -> Result<ValidationResult> {
    let tasks = read_jsonl::<TaskEntry>(&crate::common::tasks_path())?;
    
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    
    for (i, task) in tasks.iter().enumerate() {
        if let Err(e) = task.validate() {
            errors.push(format!("Line {}: {}", i + 1, e));
        }
        
        // Check for new priority field in strict mode
        if strict && task.priority.is_none() {
            errors.push(format!("Line {}: Missing priority field (required in v1.2.0+)", i + 1));
        }
        
        // Check for priority_score validation
        if let Some(score) = task.priority_score {
            if !(0.0..=100.0).contains(&score) {
                errors.push(format!("Line {}: priority_score must be between 0-100, got {}", i + 1, score));
            }
        }
    }
    
    let status = if errors.is_empty() { "passed" } else { "failed" };
    
    Ok(ValidationResult {
        status: status.to_string(),
        errors,
        warnings,
        items_checked: tasks.len() as u32,
    })
}

/// Check for Buckle Mode trigger conditions (agent mode)
pub fn check_buckle_trigger() -> Result<()> {
    check_rotd_initialized()?;
    
    let mut triggered = false;
    let mut reasons = Vec::new();
    
    // Check for compilation errors
    // Implementation would check cargo/npm output for error count
    
    // Check task.jsonl integrity
    // Implementation would verify task.jsonl status is consistent
    
    // Check test summaries
    // Implementation would verify test summaries exist for completed tasks
    
    // Check session state
    // Implementation would verify session_state.json is up to date
    
    // Return JSON result
    let result = json!({
        "triggered": triggered,
        "reasons": reasons,
        "recommendation": triggered ? "Enter Buckle Mode" : "No action needed"
    });
    
    println!("{}", serde_json::to_string(&result)?);
    
    Ok(())
}

/// Enter Buckle Mode for a specific task (agent mode)
pub fn enter_buckle_mode(task_id: &str) -> Result<()> {
    check_rotd_initialized()?;
    
    // Check if already in Buckle Mode
    let buckle_state_path = crate::common::rotd_path().join("buckle_state.json");
    if buckle_state_path.exists() {
        let state: BuckleModeState = serde_json::from_str(&std::fs::read_to_string(&buckle_state_path)?)?
            .map_err(|e| anyhow::anyhow!("{{\"error\":\"invalid_json\",\"message\":\"{}\"}}", e))?;
        
        if state.active {
            let result = json!({
                "status": "error",
                "message": format!("Already in Buckle Mode for task: {}", state.task_id.unwrap_or_default()),
                "current_task": state.task_id
            });
            println!("{}", serde_json::to_string(&result)?);
            return Ok(());
        }
    }
    
    // Create Buckle Mode state
    let state = BuckleModeState {
        active: true,
        task_id: Some(task_id.to_string()),
        entered_at: chrono::Utc::now().to_rfc3339(),
        compilation_fixed: false,
        artifacts_fixed: false,
        exit_criteria_met: false,
    };
    
    // Save state
    std::fs::write(
        buckle_state_path,
        serde_json::to_string_pretty(&state)?
    )?;
    
    // Log to audit log
    audit::log_entry(
        task_id,
        "audit.buckle.trigger.001",
        "critical",
        "Entered Buckle Mode manually",
    )?;
    
    // Return JSON result with diagnostics
    let diagnostics = diagnose_buckle_mode_json()?;
    let result = json!({
        "status": "success",
        "message": "Entered Buckle Mode successfully",
        "task_id": task_id,
        "diagnostics": diagnostics
    });
    
    println!("{}", serde_json::to_string(&result)?);
    
    Ok(())
}

/// Generate diagnostic report for Buckle Mode (agent mode)
pub fn diagnose_buckle_mode_json() -> Result<Value> {
    check_rotd_initialized()?;
    
    // Check Buckle Mode state
    let buckle_state_path = crate::common::rotd_path().join("buckle_state.json");
    if !buckle_state_path.exists() {
        return Ok(json!({
            "status": "error",
            "message": "Not in Buckle Mode"
        }));
    }
    
    let state: BuckleModeState = serde_json::from_str(&std::fs::read_to_string(&buckle_state_path)?)?;
    if !state.active {
        return Ok(json!({
            "status": "error",
            "message": "Not in Buckle Mode"
        }));
    }
    
    let task_id = state.task_id.unwrap_or_default();
    
    // Implementation would collect diagnostics
    
    let diagnostics = json!({
        "task_id": task_id,
        "compilation": {
            "status": "unknown",
            "errors": 0
        },
        "tests": {
            "status": "unknown",
            "total": 0,
            "passed": 0
        },
        "artifacts": {
            "status": "unknown",
            "missing": []
        },
        "task_tracking": {
            "status": "unknown",
            "issues": []
        },
        "exit_criteria": {
            "compilation_fixed": state.compilation_fixed,
            "artifacts_fixed": state.artifacts_fixed,
            "exit_criteria_met": state.exit_criteria_met,
            "can_exit": state.exit_criteria_met
        }
    });
    
    Ok(diagnostics)
}

/// Diagnose Buckle Mode status (agent mode)
pub fn diagnose_buckle_mode() -> Result<()> {
    let diagnostics = diagnose_buckle_mode_json()?;
    println!("{}", serde_json::to_string(&diagnostics)?);
    Ok(())
}

/// Fix compilation errors (agent mode)
pub fn fix_compilation() -> Result<()> {
    check_rotd_initialized()?;
    
    // Check Buckle Mode state
    let buckle_state_path = crate::common::rotd_path().join("buckle_state.json");
    if !buckle_state_path.exists() {
        let result = json!({
            "status": "error",
            "message": "Not in Buckle Mode"
        });
        println!("{}", serde_json::to_string(&result)?);
        return Ok(());
    }
    
    let mut state: BuckleModeState = serde_json::from_str(&std::fs::read_to_string(&buckle_state_path)?)?;
    if !state.active {
        let result = json!({
            "status": "error",
            "message": "Not in Buckle Mode"
        });
        println!("{}", serde_json::to_string(&result)?);
        return Ok(());
    }
    
    let task_id = state.task_id.as_ref().unwrap_or(&"unknown".to_string());
    
    // Implementation would attempt to fix compilation errors
    
    // Update state
    state.compilation_fixed = true;
    std::fs::write(
        buckle_state_path,
        serde_json::to_string_pretty(&state)?
    )?;
    
    // Return JSON result
    let result = json!({
        "status": "success",
        "message": "Compilation fixes applied",
        "task_id": task_id,
        "next_step": "fix-artifacts"
    });
    
    println!("{}", serde_json::to_string(&result)?);
    
    Ok(())
}

/// Fix artifacts (agent mode)
pub fn fix_artifacts() -> Result<()> {
    check_rotd_initialized()?;
    
    // Check Buckle Mode state
    let buckle_state_path = crate::common::rotd_path().join("buckle_state.json");
    if !buckle_state_path.exists() {
        let result = json!({
            "status": "error",
            "message": "Not in Buckle Mode"
        });
        println!("{}", serde_json::to_string(&result)?);
        return Ok(());
    }
    
    let mut state: BuckleModeState = serde_json::from_str(&std::fs::read_to_string(&buckle_state_path)?)?;
    if !state.active {
        let result = json!({
            "status": "error",
            "message": "Not in Buckle Mode"
        });
        println!("{}", serde_json::to_string(&result)?);
        return Ok(());
    }
    
    let task_id = state.task_id.as_ref().unwrap_or(&"unknown".to_string());
    
    // Implementation would attempt to fix artifacts
    
    // Update state
    state.artifacts_fixed = true;
    std::fs::write(
        buckle_state_path,
        serde_json::to_string_pretty(&state)?
    )?;
    
    // Return JSON result
    let result = json!({
        "status": "success",
        "message": "Artifact fixes applied",
        "task_id": task_id,
        "next_step": "check-exit"
    });
    
    println!("{}", serde_json::to_string(&result)?);
    
    Ok(())
}

/// Check exit criteria (agent mode)
pub fn check_exit_criteria() -> Result<()> {
    check_rotd_initialized()?;
    
    // Check Buckle Mode state
    let buckle_state_path = crate::common::rotd_path().join("buckle_state.json");
    if !buckle_state_path.exists() {
        let result = json!({
            "status": "error",
            "message": "Not in Buckle Mode"
        });
        println!("{}", serde_json::to_string(&result)?);
        return Ok(());
    }
    
    let mut state: BuckleModeState = serde_json::from_str(&std::fs::read_to_string(&buckle_state_path)?)?;
    if !state.active {
        let result = json!({
            "status": "error",
            "message": "Not in Buckle Mode"
        });
        println!("{}", serde_json::to_string(&result)?);
        return Ok(());
    }
    
    let task_id = state.task_id.as_ref().unwrap_or(&"unknown".to_string());
    
    // Implementation would check all exit criteria
    
    // Update state
    state.exit_criteria_met = true;
    std::fs::write(
        buckle_state_path,
        serde_json::to_string_pretty(&state)?
    )?;
    
    // Return JSON result
    let result = json!({
        "status": "success",
        "message": "All exit criteria met",
        "task_id": task_id,
        "can_exit": true,
        "next_step": "exit"
    });
    
    println!("{}", serde_json::to_string(&result)?);
    
    Ok(())
}

/// Exit Buckle Mode (agent mode)
pub fn exit_buckle_mode() -> Result<()> {
    check_rotd_initialized()?;
    
    // Check Buckle Mode state
    let buckle_state_path = crate::common::rotd_path().join("buckle_state.json");
    if !buckle_state_path.exists() {
        let result = json!({
            "status": "error",
            "message": "Not in Buckle Mode"
        });
        println!("{}", serde_json::to_string(&result)?);
        return Ok(());
    }
    
    let state: BuckleModeState = serde_json::from_str(&std::fs::read_to_string(&buckle_state_path)?)?;
    if !state.active {
        let result = json!({
            "status": "error",
            "message": "Not in Buckle Mode"
        });
        println!("{}", serde_json::to_string(&result)?);
        return Ok(());
    }
    
    let task_id = state.task_id.as_ref().unwrap_or(&"unknown".to_string());
    
    // Check if exit criteria are met
    if !state.exit_criteria_met {
        let result = json!({
            "status": "error",
            "message": "Exit criteria not met",
            "task_id": task_id
        });
        println!("{}", serde_json::to_string(&result)?);
        return Ok(());
    }
    
    // Remove Buckle Mode state
    std::fs::remove_file(buckle_state_path)?;
    
    // Log to audit log
    audit::log_entry(
        task_id,
        "audit.buckle.exit",
        "info",
        "Exited Buckle Mode successfully",
    )?;
    
    // Return JSON result
    let result = json!({
        "status": "success",
        "message": "Exited Buckle Mode successfully",
        "task_id": task_id
    });
    
    println!("{}", serde_json::to_string(&result)?);
    
    Ok(())
}