use anyhow::Result;
use colored::*;
use std::collections::HashMap;

use crate::audit;
use crate::common::check_rotd_initialized;
use crate::fs_ops::*;
use crate::pss;
use crate::schema::*;

pub fn init(force: bool, dry_run: bool, verbose: bool) -> Result<()> {
    if dry_run {
        println!("{}", "DRY RUN MODE - No changes will be made".yellow().bold());
        println!();
    }

    let rotd_dir = crate::common::rotd_path();
    
    if rotd_dir.exists() && !force {
        if !dialoguer::Confirm::new()
            .with_prompt(format!("{} already exists. Overwrite?", ".rotd".yellow()))
            .default(false)
            .interact()?
        {
            println!("{}", "Initialization cancelled.".red());
            return Ok(());
        }
    }

    if dry_run {
        println!("Would create ROTD directory structure:");
        println!("  {}", ".rotd/".cyan());
        println!("  ├── {}", "tasks.jsonl".white());
        println!("  ├── {}", "session_state.json".white());
        println!("  ├── {}", "coverage_history.json".white());
        println!("  └── {}", "test_summaries/".cyan());
        return Ok(());
    }

    if rotd_dir.exists() {
        std::fs::remove_dir_all(&rotd_dir)?;
    }

    // Create directory structure
    std::fs::create_dir_all(&rotd_dir)?;
    std::fs::create_dir_all(crate::common::test_summaries_path())?;

    // Create initial files with templates
    create_initial_files(verbose)?;

    println!("{}", "✓ ROTD project initialized successfully!".green().bold());
    
    if verbose {
        show_initialization_details();
    }

    println!();
    println!("{}", "🚀 Next steps:".bold());
    println!("  1. Add your first task: {}", "echo '{{\"id\":\"1.1\",\"title\":\"Initial setup\",\"status\":\"pending\"}}' >> .rotd/tasks.jsonl".dim());
    println!("  2. Check compliance: {}", "rotd check".dim());
    println!("  3. Read methodology: {}", "cat .rotd/ROTD.md".dim());

    Ok(())
}

pub fn score(task_id: &str, format: &str, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    let score = pss::score_task(task_id)?;

    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&score)?);
        }
        "summary" => {
            print_score_summary(&score, verbose);
        }
        "table" | _ => {
            print_score_table(&[score], verbose);
        }
    }

    Ok(())
}

pub fn show_task(task_id: &str, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    let tasks: Vec<TaskEntry> = read_jsonl(&crate::common::tasks_path())?;
    let task = tasks.iter().find(|t| t.id == task_id);

    match task {
        Some(t) => {
            println!("{} {}", "Task:".bold(), t.id.cyan());
            println!("{} {}", "Title:".bold(), t.title);
            println!("{} {:?}", "Status:".bold(), format_status(&t.status));
            
            if let Some(desc) = &t.description {
                println!("{} {}", "Description:".bold(), desc);
            }
            
            if let Some(tests) = &t.tests {
                println!("{} {}", "Tests:".bold(), tests.join(", "));
            }

            if verbose {
                println!();
                println!("{}", "Detailed Information:".bold().underlined());
                
                if let Some(created) = &t.created {
                    println!("{} {}", "Created:".bold(), created.format("%Y-%m-%d %H:%M UTC"));
                }
                
                if let Some(updated) = &t.updated_at {
                    println!("{} {}", "Updated:".bold(), updated.format("%Y-%m-%d %H:%M UTC"));
                }

                // Try to load test summary
                let summary_path = crate::common::test_summary_file(task_id);
                if summary_path.exists() {
                    if let Ok(summary) = read_json::<TestSummary>(&summary_path) {
                        println!();
                        println!("{}", "Test Summary:".bold().underlined());
                        println!("  {}: {}/{}", "Results".bold(), summary.passed, summary.total_tests);
                        if let Some(coverage) = summary.coverage {
                            println!("  {}: {:.1}%", "Coverage".bold(), coverage * 100.0);
                        }
                    }
                }
            }
        }
        None => {
            println!("{}", format!("Task '{}' not found", task_id).red());
        }
    }

    Ok(())
}

pub fn show_lessons(tag_filter: Option<&str>, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    let lessons: Vec<LessonLearned> = read_jsonl(&crate::common::lessons_path())?;

    let filtered_lessons: Vec<_> = if let Some(tag) = tag_filter {
        lessons.iter().filter(|l| l.tags.contains(&tag.to_string())).collect()
    } else {
        lessons.iter().collect()
    };

    if filtered_lessons.is_empty() {
        println!("{}", "No lessons learned found.".yellow());
        return Ok(());
    }

    println!("{}", "Lessons Learned".bold().underlined());
    println!();

    for lesson in filtered_lessons {
        println!("{} {}", "ID:".bold(), lesson.id.cyan());
        println!("{} {}", "Diagnosis:".bold(), lesson.diagnosis);
        println!("{} {}", "Remediation:".bold(), lesson.remediation.green());
        
        if !lesson.tags.is_empty() {
            println!("{} {}", "Tags:".bold(), lesson.tags.join(", ").dim());
        }

        if verbose {
            if !lesson.trigger.is_empty() {
                println!("{} {}", "Triggers:".bold(), lesson.trigger.join(", "));
            }
            
            if let Some(timestamp) = &lesson.timestamp {
                println!("{} {}", "Logged:".bold(), timestamp.format("%Y-%m-%d %H:%M UTC"));
            }
        }

        println!();
    }

    Ok(())
}

pub fn show_audit(limit: usize, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    let log_lines = audit::read_audit_log(limit)?;

    if log_lines.is_empty() {
        println!("{}", "No audit entries found.".yellow());
        return Ok(());
    }

    println!("{}", "Recent Audit Log".bold().underlined());
    println!();

    for line in log_lines.iter().rev() {
        print_audit_line(line, verbose);
    }

    Ok(())
}

pub fn check(fix: bool, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    println!("{}", "ROTD Compliance Check".bold().underlined());
    println!();

    let mut issues = Vec::new();
    let mut passed = 0;
    let total_checks = 5;

    // Check 1: Required files exist
    let required_files = [
        ("tasks.jsonl", crate::common::tasks_path()),
        ("session_state.json", crate::common::session_state_path()),
        ("coverage_history.json", crate::common::coverage_history_path()),
    ];

    let mut missing_files = Vec::new();
    for (name, path) in &required_files {
        if path.exists() {
            println!("  {} {}", "✓".green(), name);
        } else {
            println!("  {} {}", "✗".red(), name);
            missing_files.push(*name);
        }
    }

    if missing_files.is_empty() {
        passed += 1;
    } else {
        issues.push(format!("Missing files: {}", missing_files.join(", ")));
    }

    // Check 2: JSONL files are valid
    match read_jsonl::<TaskEntry>(&crate::common::tasks_path()) {
        Ok(_) => {
            println!("  {} tasks.jsonl format", "✓".green());
            passed += 1;
        }
        Err(e) => {
            println!("  {} tasks.jsonl format", "✗".red());
            issues.push(format!("Invalid tasks.jsonl: {}", e));
        }
    }

    // Continue with other checks...
    let health_score = (passed as f64 / total_checks as f64) * 100.0;
    
    println!();
    let score_display = match health_score as u32 {
        90..=100 => format!("{:.0}%", health_score).green(),
        70..=89 => format!("{:.0}%", health_score).yellow(),
        _ => format!("{:.0}%", health_score).red(),
    };
    
    println!("{} {}/{} ({})", 
        "Health Score:".bold(), 
        passed, 
        total_checks,
        score_display
    );

    if !issues.is_empty() {
        println!();
        println!("{}", "Issues Found:".red().bold());
        for (i, issue) in issues.iter().enumerate() {
            println!("  {}. {}", i + 1, issue);
        }

        if fix {
            println!();
            println!("{}", "Auto-fixing issues...".yellow());
            println!("  {} Auto-fix not yet implemented", "!".yellow());
        }
    }

    Ok(())
}

pub fn completions(shell: &str) -> Result<()> {
    use clap::{Command, CommandFactory};
    use clap_complete::{generate, Shell};
    use std::io;

    let shell_type = match shell.to_lowercase().as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" | "pwsh" => Shell::PowerShell,
        _ => {
            println!("{}", "Unsupported shell. Supported: bash, zsh, fish, powershell".red());
            return Ok(());
        }
    };

    // Create a basic command for completions since we can't access Cli here
    let mut cmd = Command::new("rotd")
        .about("Runtime-Oriented Test Discipline CLI utility")
        .subcommand(Command::new("init").about("Initialize ROTD structure"))
        .subcommand(Command::new("score").about("Generate PSS score"))
        .subcommand(Command::new("check").about("Check compliance"))
        .subcommand(Command::new("agent").about("Agent commands"));
    
    generate(shell_type, &mut cmd, "rotd", &mut io::stdout());

    Ok(())
}

// Helper functions
fn create_initial_files(verbose: bool) -> Result<()> {
    use chrono::Utc;

    // Create initial task
    let initial_task = TaskEntry {
        id: "init".to_string(),
        title: "Initialize ROTD project".to_string(),
        status: TaskStatus::Complete,
        tests: None,
        description: Some("Project initialization with ROTD structure".to_string()),
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

    // Create session state
    let session_state = SessionState {
        session_id: "init".to_string(),
        timestamp: Utc::now(),
        current_task: Some("init".to_string()),
        status: "initialized".to_string(),
        deltas: None,
    };

    write_json(&crate::common::session_state_path(), &session_state)?;

    // Create coverage history
    let coverage_history = CoverageHistory {
        floor: 70.0,
        ratchet_threshold: 3.0,
        history: Vec::new(),
    };

    write_json(&crate::common::coverage_history_path(), &coverage_history)?;

    if verbose {
        println!("Created initial files:");
        println!("  - Initial task entry in tasks.jsonl");
        println!("  - Session state tracking");
        println!("  - Coverage history baseline");
    }

    Ok(())
}

fn show_initialization_details() {
    println!();
    println!("📂 Created structure:");
    println!("  {}", ".rotd/".cyan());
    println!("  ├── {}", "tasks.jsonl".white());
    println!("  ├── {}", "session_state.json".white());
    println!("  ├── {}", "coverage_history.json".white());
    println!("  └── {}", "test_summaries/".cyan());
}

fn format_status(status: &TaskStatus) -> colored::ColoredString {
    match status {
        TaskStatus::Complete => "Complete".green(),
        TaskStatus::InProgress => "In Progress".yellow(),
        TaskStatus::Pending => "Pending".blue(),
        TaskStatus::Blocked => "Blocked".red(),
        TaskStatus::Scaffolded => "Scaffolded".magenta(),
    }
}

fn print_score_summary(score: &PSSScore, verbose: bool) {
    println!("{} {}", "Task:".bold(), score.task_id.cyan());
    
    let score_color = match score.score {
        9..=10 => score.score.to_string().green(),
        7..=8 => score.score.to_string().yellow(),
        _ => score.score.to_string().red(),
    };
    
    println!("{} {}/10", "Score:".bold(), score_color);
    
    println!();
    println!("{}", "Breakdown:".bold());
    
    for (criterion, data) in &score.criteria {
        let icon = if data.score == 1 { "✓".green() } else { "✗".red() };
        println!("  {} {}: {}", icon, criterion, data.rationale);
    }

    if verbose {
        println!();
        println!("{} {}", "Timestamp:".bold(), score.timestamp.format("%Y-%m-%d %H:%M UTC"));
    }
}

fn print_score_table(scores: &[PSSScore], verbose: bool) {
    if scores.is_empty() {
        println!("{}", "No scores to display.".yellow());
        return;
    }

    println!("{}", "ROTD Progress Scores".bold().underlined());
    println!();

    // Header
    println!("{:<12} {:<8} {:<12} {:<20}", 
        "Task ID".bold(),
        "Score".bold(), 
        "Status".bold(),
        "Timestamp".bold()
    );
    println!("{}", "─".repeat(60));

    // Rows
    for score in scores {
        let score_color = match score.score {
            9..=10 => score.score.to_string().green(),
            7..=8 => score.score.to_string().yellow(), 
            _ => score.score.to_string().red(),
        };

        let status = get_status_from_score(score.score);
        let status_color = match score.score {
            9..=10 => status.green(),
            7..=8 => status.yellow(),
            _ => status.red(),
        };

        println!("{:<12} {:<8} {:<12} {:<20}",
            score.task_id.cyan(),
            score_color,
            status_color,
            score.timestamp.format("%Y-%m-%d %H:%M")
        );
    }

    if verbose && scores.len() > 1 {
        println!();
        let avg_score = scores.iter().map(|s| s.score).sum::<u32>() as f64 / scores.len() as f64;
        let avg_color = match avg_score as u32 {
            9..=10 => format!("{:.1}", avg_score).green(),
            7..=8 => format!("{:.1}", avg_score).yellow(),
            _ => format!("{:.1}", avg_score).red(),
        };
        
        println!("{} {}", "Average Score:".bold(), avg_color);
    }
}

fn get_status_from_score(score: u32) -> &'static str {
    match score {
        10 => "Perfect",
        9 => "Excellent", 
        8 => "Good",
        7 => "Fair",
        6 => "Passing",
        5 => "Poor",
        4 => "Failing",
        0..=3 => "Critical",
        _ => "Unknown",
    }
}

fn print_audit_line(line: &str, verbose: bool) {
    // Parse and colorize audit log lines
    if line.contains("[ERROR]") {
        println!("  {}", line.red());
    } else if line.contains("[WARNING]") {
        println!("  {}", line.yellow());
    } else if line.contains("[INFO]") {
        println!("  {}", line.white());
    } else {
        println!("  {}", line.dim());
    }
}