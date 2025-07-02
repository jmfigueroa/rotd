use anyhow::Result;
use chrono::Utc;
use serde_json;

use crate::audit;
use crate::common::check_rotd_initialized;
use crate::fs_ops::*;
use crate::pss;
use crate::schema::*;

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