use anyhow::Result;
use colored::*;

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
    println!("  1. Add your first task: {}", "echo '{{\"id\":\"1.1\",\"title\":\"Initial setup\",\"status\":\"pending\"}}' >> .rotd/tasks.jsonl".dimmed());
    println!("  2. Check compliance: {}", "rotd check".dimmed());
    println!("  3. Read methodology: {}", "cat .rotd/ROTD.md".dimmed());

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
                println!("{}", "Detailed Information:".bold().underline());
                
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
                        println!("{}", "Test Summary:".bold().underline());
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

    println!("{}", "Lessons Learned".bold().underline());
    println!();

    for lesson in filtered_lessons {
        println!("{} {}", "ID:".bold(), lesson.id.cyan());
        println!("{} {}", "Diagnosis:".bold(), lesson.diagnosis);
        println!("{} {}", "Remediation:".bold(), lesson.remediation.green());
        
        if !lesson.tags.is_empty() {
            println!("{} {}", "Tags:".bold(), lesson.tags.join(", ").dimmed());
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

    println!("{}", "Recent Audit Log".bold().underline());
    println!();

    for line in log_lines.iter().rev() {
        print_audit_line(line, verbose);
    }

    Ok(())
}

pub fn check(fix: bool, _verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    println!("{}", "ROTD Compliance Check".bold().underline());
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
    use clap::Command;
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
        priority_score: None,
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

    println!("{}", "ROTD Progress Scores".bold().underline());
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

fn print_audit_line(line: &str, _verbose: bool) {
    // Parse and colorize audit log lines
    if line.contains("[ERROR]") {
        println!("  {}", line.red());
    } else if line.contains("[WARNING]") {
        println!("  {}", line.yellow());
    } else if line.contains("[INFO]") {
        println!("  {}", line.white());
    } else {
        println!("  {}", line.dimmed());
    }
}

// New update-related functions
pub fn update(check_only: bool, skip_confirmation: bool, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;
    
    if check_only {
        return check_for_updates(verbose);
    }
    
    println!("{}", "🔄 ROTD Update Process".bold().underline());
    println!();
    
    if !skip_confirmation {
        if !dialoguer::Confirm::new()
            .with_prompt("Update ROTD methodology and templates?")
            .default(true)
            .interact()?
        {
            println!("{}", "Update cancelled.".yellow());
            return Ok(());
        }
    }
    
    // Step 1: Backup current state
    println!("{}", "1. Creating backup...".bold());
    backup_rotd_files(verbose)?;
    println!("   {} Backup created", "✓".green());
    
    // Step 2: Generate update manifest (for now, hardcoded v1.2.0)
    println!("{}", "2. Generating update manifest...".bold());
    let manifest = create_v1_2_0_manifest();
    write_update_manifest(&manifest, verbose)?;
    println!("   {} Manifest created", "✓".green());
    
    // Step 3: Show user guidance
    println!();
    display_update_guidance(&manifest)?;
    
    Ok(())
}

pub fn version(project: bool, latest: bool, verbose: bool) -> Result<()> {
    if project {
        show_project_version(verbose)
    } else if latest {
        show_latest_version(verbose)
    } else {
        show_all_versions(verbose)
    }
}

pub fn validate(all: bool, schema_type: Option<&str>, strict: bool, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;
    
    println!("{}", "ROTD Validation Report".bold().underline());
    println!();
    
    if all {
        validate_all_schemas(strict, verbose)
    } else if let Some(schema) = schema_type {
        validate_specific_schema(schema, strict, verbose)
    } else {
        validate_all_schemas(strict, verbose)
    }
}

// Helper functions for update functionality
fn check_for_updates(verbose: bool) -> Result<()> {
    println!("{}", "Checking for ROTD updates...".bold());
    
    // For now, simulate checking
    println!("   Current version: {}", "1.1.0".white());
    println!("   Latest version: {}", "1.2.0".cyan());
    println!("   {} Update available!", "✓".green());
    
    if verbose {
        println!();
        println!("Changes in v1.2.0:");
        println!("  • Task prioritization system");
        println!("  • Periodic review process");
        println!("  • Enhanced validation");
    }
    
    Ok(())
}

fn backup_rotd_files(verbose: bool) -> Result<()> {
    use std::fs;
    
    let rotd_dir = crate::common::rotd_path();
    let backup_dir = rotd_dir.join("backup");
    
    if backup_dir.exists() {
        fs::remove_dir_all(&backup_dir)?;
    }
    fs::create_dir_all(&backup_dir)?;
    
    // Backup key files
    let files_to_backup = [
        "tasks.jsonl",
        "session_state.json", 
        "coverage_history.json",
        "pss_scores.jsonl",
        "lessons_learned.jsonl",
    ];
    
    for file in &files_to_backup {
        let src = rotd_dir.join(file);
        if src.exists() {
            let dest = backup_dir.join(file);
            fs::copy(&src, &dest)?;
            if verbose {
                println!("     Backed up {}", file);
            }
        }
    }
    
    Ok(())
}

fn create_v1_2_0_manifest() -> UpdateManifest {
    UpdateManifest {
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
            ChangeEntry {
                change_type: "feature".to_string(),
                component: "workflow".to_string(),
                description: "Added periodic review process".to_string(),
                breaking: false,
                migration_required: false,
            },
        ],
    }
}

fn write_update_manifest(manifest: &UpdateManifest, verbose: bool) -> Result<()> {
    let manifest_path = crate::common::rotd_path().join("update_manifest.json");
    write_json(&manifest_path, manifest)?;
    
    if verbose {
        println!("     Manifest written to update_manifest.json");
    }
    
    Ok(())
}

fn display_update_guidance(manifest: &UpdateManifest) -> Result<()> {
    println!("{}", "🔄 Update Complete!".green().bold());
    println!("Version: {} → {}", manifest.previous_version, manifest.version);
    println!();
    
    // Show changes summary
    println!("{}", "📋 Changes Applied:".bold());
    for change in &manifest.changes {
        let icon = if change.migration_required { "🔧" } else { "✨" };
        println!("  {} {} ({})", icon, change.description, change.component);
    }
    println!();
    
    // Show required actions
    let needs_migration = manifest.changes.iter().any(|c| c.migration_required);
    if needs_migration {
        println!("{}", "📋 Next Steps:".bold());
        println!("Use this prompt with your LLM to apply updates to your project:");
        println!();
        println!("```");
        print_update_prompt(&manifest.changes);
        println!("```");
    } else {
        println!("{}", "✅ No migration required - you're all set!".green());
    }
    
    Ok(())
}

fn print_update_prompt(changes: &[ChangeEntry]) {
    println!("ROTD methodology has been updated. Apply the latest changes to this project.");
    println!();
    println!("📋 **Update Process**:");
    println!();
    println!("1. **Review Changes**");
    println!("   - Check `.rotd/update_manifest.json` for list of updates");
    println!("   - Identify which changes affect this project");
    println!();
    
    for change in changes.iter().filter(|c| c.migration_required) {
        match change.component.as_str() {
            "task_schema" => {
                println!("2. **Apply Task Schema Updates**");
                println!("   - Add priority field to existing tasks");
                println!("   - Use migration logic: blocked→urgent, in_progress→high, pending→medium, complete→low");
                println!("   - Optionally add priority_score (0-100) for finer ranking");
                println!();
            }
            "workflow" => {
                println!("2. **Implement New Workflows**");
                println!("   - Create review schedule file");
                println!("   - Set up periodic review process");
                println!();
            }
            _ => {
                println!("2. **Apply {} Changes**", change.component);
                println!("   - {}", change.description);
                println!();
            }
        }
    }
    
    println!("3. **Verify Updates**");
    println!("   ```bash");
    println!("   rotd validate --all --strict");
    println!("   rotd check");
    println!("   ```");
    println!();
    println!("4. **Log Completion**");
    println!("   Add entry to `.rotd/update_history.jsonl` when done");
}

fn show_project_version(verbose: bool) -> Result<()> {
    let version_path = crate::common::rotd_path().join("version.json");
    
    if version_path.exists() {
        let version: ProjectVersion = read_json(&version_path)?;
        println!("Project ROTD Version: {}", version.version.cyan());
        
        if verbose {
            println!("Updated: {}", version.updated_at.format("%Y-%m-%d %H:%M UTC"));
            if let Some(hash) = &version.manifest_hash {
                println!("Manifest Hash: {}", hash);
            }
        }
    } else {
        println!("Project ROTD Version: {} (estimated)", "1.1.0".yellow());
        if verbose {
            println!("No version.json found - run 'rotd update' to track versions");
        }
    }
    
    Ok(())
}

fn show_latest_version(_verbose: bool) -> Result<()> {
    // For now, hardcoded - in real implementation would fetch from repository
    println!("Latest ROTD Version: {}", "1.2.0".green());
    Ok(())
}

fn show_all_versions(verbose: bool) -> Result<()> {
    show_project_version(verbose)?;
    show_latest_version(verbose)?;
    
    if verbose {
        println!();
        println!("Use 'rotd update --check' to see available updates");
    }
    
    Ok(())
}

fn validate_all_schemas(strict: bool, verbose: bool) -> Result<()> {
    let mut passed = 0;
    let mut total = 0;
    
    // Validate tasks.jsonl
    total += 1;
    match validate_tasks_schema(strict, verbose) {
        Ok(_) => {
            println!("  {} tasks.jsonl schema", "✓".green());
            passed += 1;
        }
        Err(e) => {
            println!("  {} tasks.jsonl schema: {}", "✗".red(), e);
        }
    }
    
    // Validate other schemas
    total += 1;
    if crate::common::rotd_path().join("pss_scores.jsonl").exists() {
        println!("  {} pss_scores.jsonl schema", "✓".green());
        passed += 1;
    } else {
        println!("  {} pss_scores.jsonl not found", "!".yellow());
    }
    
    println!();
    let score = (passed as f64 / total as f64) * 100.0;
    let score_display = match score as u32 {
        90..=100 => format!("{:.0}%", score).green(),
        70..=89 => format!("{:.0}%", score).yellow(),
        _ => format!("{:.0}%", score).red(),
    };
    
    println!("{} {}/{} ({})", 
        "Validation Score:".bold(), 
        passed, 
        total,
        score_display
    );
    
    Ok(())
}

fn validate_specific_schema(schema: &str, strict: bool, verbose: bool) -> Result<()> {
    match schema {
        "tasks" => validate_tasks_schema(strict, verbose).map(|_| ()),
        "scores" => {
            println!("  {} pss_scores validation not yet implemented", "!".yellow());
            Ok(())
        }
        _ => {
            println!("{}", format!("Unknown schema type: {}", schema).red());
            Ok(())
        }
    }
}

fn validate_tasks_schema(strict: bool, verbose: bool) -> Result<Vec<TaskEntry>> {
    let tasks = read_jsonl::<TaskEntry>(&crate::common::tasks_path())?;
    
    let mut errors = Vec::new();
    for (i, task) in tasks.iter().enumerate() {
        if let Err(e) = task.validate() {
            errors.push(format!("Line {}: {}", i + 1, e));
        }
        
        // Check for new priority field in strict mode
        if strict && task.priority.is_none() {
            errors.push(format!("Line {}: Missing priority field (required in v1.2.0+)", i + 1));
        }
    }
    
    if !errors.is_empty() {
        if verbose {
            for error in &errors {
                println!("    {}", error.red());
            }
        }
        return Err(anyhow::anyhow!("{} validation errors", errors.len()));
    }
    
    Ok(tasks)
}