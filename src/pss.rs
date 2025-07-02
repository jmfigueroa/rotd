use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;

use crate::fs_ops::{read_jsonl, read_json, append_jsonl};
use crate::schema::{TaskEntry, TestSummary, PSSScore, CriterionScore, CoverageHistory};

pub fn score_task(task_id: &str) -> Result<PSSScore> {
    let mut criteria = HashMap::new();

    // Load relevant data
    let tasks: Vec<TaskEntry> = read_jsonl(&crate::common::tasks_path())?;
    let task = tasks.iter().find(|t| t.id == task_id);
    
    let test_summary = load_test_summary(task_id).ok();
    let coverage_history = read_json::<CoverageHistory>(&crate::common::coverage_history_path()).ok();

    // 1. LLM Engagement
    let engaged = task.map_or(false, |t| {
        matches!(t.status, crate::schema::TaskStatus::InProgress | crate::schema::TaskStatus::Complete)
    });
    criteria.insert("llm_engaged".to_string(), CriterionScore {
        score: if engaged { 1 } else { 0 },
        rationale: format!("Task {} status: {:?}", task_id, 
            task.map(|t| &t.status).unwrap_or(&crate::schema::TaskStatus::Pending)),
    });

    // 2. Compiles
    let compiles = check_compiles();
    criteria.insert("compiles".to_string(), CriterionScore {
        score: if compiles { 1 } else { 0 },
        rationale: if compiles {
            "Project compiles cleanly".to_string()
        } else {
            "Compilation errors detected".to_string()
        },
    });

    // 3. Core Implementation
    let implemented = task.map_or(false, |t| {
        matches!(t.status, crate::schema::TaskStatus::Complete)
    });
    criteria.insert("core_impl".to_string(), CriterionScore {
        score: if implemented { 1 } else { 0 },
        rationale: format!("Task status: {:?}", 
            task.map(|t| &t.status).unwrap_or(&crate::schema::TaskStatus::Pending)),
    });

    // 4. Tests Written
    let tests_written = test_summary.as_ref()
        .map_or(false, |ts| ts.total_tests > 0);
    criteria.insert("tests_written".to_string(), CriterionScore {
        score: if tests_written { 1 } else { 0 },
        rationale: format!("Test summary shows {} tests", 
            test_summary.as_ref().map(|ts| ts.total_tests).unwrap_or(0)),
    });

    // 5. Tests Pass
    let tests_pass = if let Some(ts) = &test_summary {
        let pass_rate = ts.passed as f64 / ts.total_tests as f64;
        pass_rate >= 0.7
    } else {
        false
    };
    criteria.insert("tests_pass".to_string(), CriterionScore {
        score: if tests_pass { 1 } else { 0 },
        rationale: if let Some(ts) = &test_summary {
            let pass_rate = (ts.passed as f64 / ts.total_tests as f64) * 100.0;
            format!("Pass rate: {:.1}% ({} threshold)", pass_rate, 
                if pass_rate >= 70.0 { "meets 70%" } else { "below 70%" })
        } else {
            "No test summary available".to_string()
        },
    });

    // 6. Documentation Maintained
    criteria.insert("doc_maintained".to_string(), CriterionScore {
        score: 1, // Placeholder
        rationale: "Documentation maintained (placeholder check)".to_string(),
    });

    // 7. Stub-Free
    let stubs_remaining = check_stubs_remaining();
    criteria.insert("stub_free".to_string(), CriterionScore {
        score: if stubs_remaining { 0 } else { 1 },
        rationale: if stubs_remaining {
            "Stubs detected in codebase".to_string()
        } else {
            "No stubs detected".to_string()
        },
    });

    // 8. History Maintained
    let history_maintained = test_summary.is_some() && task.is_some();
    criteria.insert("history_maintained".to_string(), CriterionScore {
        score: if history_maintained { 1 } else { 0 },
        rationale: format!("Test summary: {}, Task in jsonl: {}", 
            if test_summary.is_some() { "✓" } else { "✗" },
            if task.is_some() { "✓" } else { "✗" }),
    });

    // 9. QTS Floor Met
    if let (Some(coverage_hist), Some(ts)) = (&coverage_history, &test_summary) {
        if let Some(coverage) = ts.coverage {
            let current_coverage = coverage * 100.0;
            let floor_met = current_coverage >= coverage_hist.floor;
            criteria.insert("qts_floor".to_string(), CriterionScore {
                score: if floor_met { 1 } else { 0 },
                rationale: format!("Coverage {:.1}% vs floor {:.1}%", 
                    current_coverage, coverage_hist.floor),
            });
        } else {
            criteria.insert("qts_floor".to_string(), CriterionScore {
                score: 0,
                rationale: "No coverage data in test summary".to_string(),
            });
        }
    } else {
        criteria.insert("qts_floor".to_string(), CriterionScore {
            score: 0,
            rationale: "Coverage data not available".to_string(),
        });
    }

    // 10. QTS Ratchet
    if let (Some(coverage_hist), Some(ts)) = (&coverage_history, &test_summary) {
        if let Some(coverage) = ts.coverage {
            let current_coverage = coverage * 100.0;
            let headroom = current_coverage - coverage_hist.floor;
            let ratchet_triggered = headroom > coverage_hist.ratchet_threshold;
            criteria.insert("qts_ratchet".to_string(), CriterionScore {
                score: if ratchet_triggered { 1 } else { 0 },
                rationale: format!("Headroom {:.1}% {} {:.1}% threshold", 
                    headroom, 
                    if ratchet_triggered { "triggers" } else { "below" },
                    coverage_hist.ratchet_threshold),
            });
        } else {
            criteria.insert("qts_ratchet".to_string(), CriterionScore {
                score: 0,
                rationale: "No coverage data in test summary".to_string(),
            });
        }
    } else {
        criteria.insert("qts_ratchet".to_string(), CriterionScore {
            score: 0,
            rationale: "Coverage data not available for ratchet calculation".to_string(),
        });
    }

    let total_score = criteria.values().map(|c| c.score).sum();

    Ok(PSSScore {
        task_id: task_id.to_string(),
        score: total_score,
        timestamp: Utc::now(),
        criteria,
    })
}

pub fn save_score(score: &PSSScore, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("Would save PSS score: {}", serde_json::to_string_pretty(score)?);
        return Ok(());
    }

    append_jsonl(&crate::common::pss_scores_path(), score)
}

fn load_test_summary(task_id: &str) -> Result<TestSummary> {
    read_json(&crate::common::test_summary_file(task_id))
}

fn check_compiles() -> bool {
    // Check for package.json (Node.js/TypeScript)
    if std::path::Path::new("package.json").exists() {
        return std::process::Command::new("npm")
            .args(&["run", "typecheck"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);
    }
    
    // Check for Cargo.toml (Rust)
    if std::path::Path::new("Cargo.toml").exists() {
        return std::process::Command::new("cargo")
            .args(&["check"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);
    }
    
    // Default assume compilation passes
    true
}

pub fn check_stubs_remaining() -> bool {
    use walkdir::WalkDir;
    
    let stub_patterns = ["#[rotd_stub]", "TODO(", "unimplemented!", "todo!", "throw new Error(\"TODO\")"];
    
    for entry in WalkDir::new("src").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if matches!(ext.to_str(), Some("rs") | Some("ts") | Some("tsx") | Some("js") | Some("jsx")) {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        // Skip checking this file's pattern definition line
                        if entry.path().ends_with("pss.rs") {
                            // Check for stubs but exclude the pattern definition line
                            for (_line_num, line) in content.lines().enumerate() {
                                if line.contains("let stub_patterns") {
                                    continue;
                                }
                                for pattern in &stub_patterns {
                                    if line.contains(pattern) {
                                        return true;
                                    }
                                }
                            }
                        } else {
                            for pattern in &stub_patterns {
                                if content.contains(pattern) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    false
}