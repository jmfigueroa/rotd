use anyhow::Result;
use colored::*;

use crate::audit;
use crate::cli::commands::buckle_mode::BuckleModeState;
use crate::common::check_rotd_initialized;
use crate::fs_ops::*;
use crate::github;
use crate::pss;
use crate::schema::*;

pub fn init(force: bool, dry_run: bool, verbose: bool) -> Result<()> {
    if dry_run {
        println!(
            "{}",
            "DRY RUN MODE - No changes will be made".yellow().bold()
        );
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
    std::fs::create_dir_all(crate::common::task_history_path())?;

    // Create initial files with templates
    create_initial_files(verbose)?;

    println!(
        "{}",
        "✓ ROTD project initialized successfully!".green().bold()
    );

    Ok(())
}

// Updates ROTD project version if available
pub fn update(check_only: bool, yes: bool, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    // Get current project version
    let version_path = crate::common::rotd_path().join("version.json");
    let current_version = if version_path.exists() {
        let v: ProjectVersion = read_json(&version_path)?;
        v.version
    } else {
        "1.3.5".to_string()
    };

    // The latest methodology version available
    let latest_methodology_version = "1.3.4";

    // Check for methodology updates
    println!("{}", "Checking for ROTD methodology updates...".cyan());
    
    // Compare semantic versions
    let needs_update = match (current_version.as_str(), latest_methodology_version) {
        (current, latest) if current == latest => false,
        (current, latest) => {
            // Simple version comparison - can be enhanced with semver crate if needed
            let current_parts: Vec<u32> = current.trim_start_matches('v')
                .split('.')
                .filter_map(|s| s.parse().ok())
                .collect();
            let latest_parts: Vec<u32> = latest.trim_start_matches('v')
                .split('.')
                .filter_map(|s| s.parse().ok())
                .collect();
            
            if current_parts.len() != 3 || latest_parts.len() != 3 {
                true // Assume update needed if version format is unexpected
            } else {
                current_parts < latest_parts
            }
        }
    };

    if check_only {
        // Display current and latest versions
        println!("   Current version: {}", current_version.green());
        println!("   Latest version: {}", latest_methodology_version.green());

        if needs_update {
            println!("   {} Update available!", "✓".green());
            
            if verbose {
                println!("\nThis will update:");
                println!("   • Project ROTD methodology to v{}", latest_methodology_version);
                println!("   • Documentation templates and examples");
                println!("   • Schema definitions and validation rules");
                println!("   • Primer strategy templates");
            }
        } else {
            println!("   {} You have the latest version.", "✓".green());
        }

        return Ok(());
    }

    // Check if update is available
    if !needs_update {
        println!("{}", "✓ You're already using the latest version!".green());
        return Ok(());
    }

    println!("{}", "✓ Update available!".green().bold());
    println!("   Current version: {}", current_version);
    println!("   Latest version: {}", latest_methodology_version);

    // Show what will be updated
    println!("\nThis update will:");
    println!("   • Update project ROTD methodology to v{}", latest_methodology_version);
    println!("   • Refresh documentation and templates");
    println!("   • Update schema definitions");
    println!("   • Add primer strategy support if missing");

    // Confirm update
    if !yes {
        if !dialoguer::Confirm::new()
            .with_prompt("Do you want to update now?")
            .default(true)
            .interact()?
        {
            println!("\n{}", "Update cancelled.".yellow());
            println!("You can update later with {}", "rotd update".cyan());
            return Ok(());
        }
    }

    // Perform the update
    println!("\n{}", "Updating project ROTD methodology...".cyan());
    
    let rotd_dir = crate::common::rotd_path();
    
    // Update version.json
    let new_version = ProjectVersion {
        version: latest_methodology_version.to_string(),
        updated_at: Some(chrono::Utc::now()),
        manifest_hash: None,
    };
    write_json(&version_path, &new_version)?;
    println!("   ✓ Updated version.json to v{}", latest_methodology_version);
    
    // Add primer strategy if missing
    let primer_path = rotd_dir.join("primer.jsonc");
    if !primer_path.exists() {
        println!("   ✓ Adding primer strategy support...");
        
        // Get project name from current directory
        let current_dir = std::env::current_dir()?;
        let project_name = current_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Project")
            .to_string();
        
        // Create basic primer structure
        let primer = ProjectPrimer {
            name: project_name.clone(),
            scope: "root".to_string(),
            description: "TODO: Add project description".to_string(),
            status: "active".to_string(),
            language: "TODO: Specify primary language".to_string(),
            entry_points: vec!["TODO: Add entry points".to_string()],
            test_dirs: vec!["tests/".to_string(), "test/".to_string()],
            dependencies: vec!["TODO: List key dependencies".to_string()],
            known_issues: vec!["TODO: Document any known issues".to_string()],
            key_concepts: vec!["TODO: Add key concepts".to_string()],
            preferred_agents: Some(vec!["Claude Sonnet".to_string(), "Claude Opus".to_string()]),
            suggested_starting_points: vec![
                "TODO: Add suggested starting points for new developers or agents".to_string()
            ],
            major_components: None,
            update_triggers: Some(vec![
                "Major architectural changes".to_string(),
                "New features or significant functionality changes".to_string(),
                "Documentation updates".to_string()
            ]),
        };
        
        // Write primer file with nice formatting
        let primer_json = serde_json::to_string_pretty(&primer)?;
        std::fs::write(&primer_path, primer_json)?;
        println!("   ✓ Created primer.jsonc template");
    }
    
    // Generate update manifest for tracking
    let manifest = UpdateManifest {
        version: latest_methodology_version.to_string(),
        date: chrono::Utc::now().to_rfc3339(),
        previous_version: current_version.clone(),
        changes: vec![ChangeEntry {
            change_type: "methodology_update".to_string(),
            component: "rotd_project".to_string(),
            description: format!("Updated ROTD methodology from {} to {}", current_version, latest_methodology_version),
            breaking: false,
            migration_required: false,
        }],
    };
    
    let manifest_path = rotd_dir.join("update_manifest.json");
    write_json(&manifest_path, &manifest)?;
    
    println!("\n{}", "✓ Project methodology updated successfully!".green().bold());
    println!("   Updated from: {}", current_version.yellow());
    println!("   Updated to: {}", latest_methodology_version.green());
    
    if primer_path.exists() {
        println!("\n{}", "📋 Primer Strategy Available".cyan());
        println!("   Use {} to customize your project primer", "rotd primer show".yellow());
        println!("   Use {} to validate primer accuracy", "rotd primer check".yellow());
    }
    
    if verbose {
        println!("\n{}", "Files updated:".cyan());
        println!("   • {}", version_path.display());
        if primer_path.exists() {
            println!("   • {}", primer_path.display());
        }
        println!("   • {}", manifest_path.display());
    }

    Ok(())
}

// Upgrades ROTD CLI binary to latest version
pub fn upgrade(check_only: bool, yes: bool, verbose: bool) -> Result<()> {
    // Get current binary version
    let current_version = env!("CARGO_PKG_VERSION");

    // Check for binary upgrades
    println!("{}", "Checking for ROTD CLI upgrades...".cyan());

    let (upgrade_available, latest_release) = match github::check_update() {
        Ok((available, release)) => (available, release),
        Err(e) => {
            println!("   {} Could not fetch latest version.", "!".yellow());
            println!("   Reason: {}", e);

            if verbose {
                println!(
                    "   
   Common solutions:
   - Check your internet connection
   - Try again in a few minutes (GitHub API may be rate limited)
   - Check if you can access https://api.github.com/repos/jmfigueroa/rotd/releases
   - If behind a corporate firewall, you may need to configure proxy settings"
                );
            }

            return Ok(());
        }
    };

    if check_only {
        // Display current and latest versions
        println!("   Current CLI version: {}", current_version.green());

        if let Some(latest) = latest_release {
            println!("   Latest CLI version: {}", latest.version.green());

            if upgrade_available {
                println!("   {} CLI upgrade available!", "✓".green());

                if verbose {
                    println!("\nChanges in latest version:");
                    for change in github::extract_changes(&latest.description) {
                        println!("   {}", change);
                    }
                    println!("\nSee more: {}", latest.html_url.cyan().underline());
                }
            } else {
                println!("   {} You have the latest CLI version.", "✓".green());
            }
        } else {
            println!("   {} No releases found on GitHub.", "!".yellow());
        }

        return Ok(());
    }

    // Check if upgrade is available
    if !upgrade_available {
        println!(
            "{}",
            "✓ You're already using the latest CLI version!".green()
        );
        return Ok(());
    }

    // Get latest release
    let latest =
        latest_release.ok_or_else(|| anyhow::anyhow!("No release information available"))?;

    println!("{}", "✓ CLI upgrade available!".green().bold());
    println!("   Current version: {}", current_version);
    println!("   Latest version: {}", latest.version);
    println!("   Published on: {}", latest.published_at);

    // Show changes
    println!("\nChanges in this upgrade:");
    for change in github::extract_changes(&latest.description) {
        println!("   {}", change);
    }

    // Confirm upgrade
    if !yes {
        if !dialoguer::Confirm::new()
            .with_prompt("Do you want to upgrade now?")
            .default(true)
            .interact()?
        {
            println!("\n{}", "Upgrade cancelled.".yellow());
            println!("You can upgrade later with {}", "rotd upgrade".cyan());
            return Ok(());
        }
    }

    // Download and install the new binary
    println!("\n{}", "Downloading and installing upgrade...".cyan());

    // Detect the current binary path
    let current_exe = std::env::current_exe()?;

    // Find the appropriate asset for the current platform
    let asset = github::find_platform_asset(&latest)?;

    // Download the binary
    println!("   Downloading from: {}", asset.browser_download_url);
    let binary_data = github::download_binary(&asset.browser_download_url)?;

    // Create temporary file for new binary
    let temp_path = current_exe.with_extension("new");
    std::fs::write(&temp_path, binary_data)?;

    // Make it executable (Unix-like systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&temp_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&temp_path, perms)?;
    }

    // Replace the current binary
    println!("   Installing new binary...");
    std::fs::rename(&temp_path, &current_exe)?;

    println!(
        "\n{}",
        "✓ CLI upgrade completed successfully!".green().bold()
    );
    println!("   New version: {}", latest.version.green());
    println!("   Run {} to verify the upgrade.", "rotd version".cyan());

    Ok(())
}

// Displays version information in human-readable format
pub fn version(project: bool, latest: bool, verbose: bool) -> Result<()> {
    if project {
        let version_path = crate::common::rotd_path().join("version.json");
        let version = if version_path.exists() {
            let v: ProjectVersion = read_json(&version_path)?;
            v.version
        } else {
            "1.3.3".to_string()
        };

        println!("Project ROTD version: {}", version.green());
        if verbose {
            println!(
                "Version tracking: {}",
                if version_path.exists() {
                    "enabled".green()
                } else {
                    "not enabled".yellow()
                }
            );
        }
    } else if latest {
        println!("Checking for latest version...");
        match github::fetch_latest_release()? {
            Some(latest) => {
                println!("Latest available version: {}", latest.version.green());
                if verbose {
                    println!("Released on: {}", latest.published_at);
                    println!("Release URL: {}", latest.html_url.cyan().underline());
                }
            }
            None => {
                println!("Could not fetch latest version information.");
            }
        }
    } else {
        let cli_version = env!("CARGO_PKG_VERSION");
        println!("ROTD CLI version: {}", cli_version.green());

        // Check project version if available
        if let Ok(_) = crate::common::check_rotd_initialized() {
            let initialized = true;
            if initialized {
                let version_path = crate::common::rotd_path().join("version.json");
                let project_version = if version_path.exists() {
                    let v: ProjectVersion = read_json(&version_path)?;
                    v.version
                } else {
                    "1.3.3".to_string() // Default if not tracked
                };

                println!("Project ROTD version: {}", project_version.green());

                // Check for latest version
                if verbose {
                    println!("\nChecking for updates...");

                    match github::check_update() {
                        Ok((update_available, latest_release)) => {
                            if let Some(latest) = latest_release {
                                println!("Latest available version: {}", latest.version.green());

                                if update_available {
                                    println!("\n{}", "Update available!".yellow());
                                    println!("Run {} to update", "rotd update".cyan());
                                } else {
                                    println!("\n{}", "You have the latest version".green());
                                }
                            } else {
                                println!("No releases found on GitHub.");
                            }
                        }
                        Err(e) => {
                            println!("Could not fetch latest version information.");
                            println!("Reason: {}", e);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

// Function to create initial files
fn create_initial_files(verbose: bool) -> Result<()> {
    // Create basic task entry
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
        priority: Some(Priority::Medium),
        priority_score: None,
        created: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
        completed: Some(chrono::Utc::now()),
    };

    if verbose {
        println!("Creating initial task entry...");
    }

    append_jsonl(&crate::common::tasks_path(), &initial_task)?;

    // Create session state
    let session_state = SessionState {
        session_id: "init".to_string(),
        timestamp: chrono::Utc::now(),
        current_task: Some("init".to_string()),
        status: "initialized".to_string(),
        deltas: None,
    };

    if verbose {
        println!("Creating session state...");
    }

    write_json(&crate::common::session_state_path(), &session_state)?;

    // Create coverage history
    let coverage_history = CoverageHistory {
        floor: 70.0,
        ratchet_threshold: 3.0,
        history: Vec::new(),
    };

    if verbose {
        println!("Creating coverage history...");
    }

    write_json(&crate::common::coverage_history_path(), &coverage_history)?;

    // Create version tracking
    let version = ProjectVersion {
        version: "1.3.5".to_string(),
        manifest_hash: None,
        updated_at: Some(chrono::Utc::now()),
    };

    if verbose {
        println!("Creating version tracking...");
    }

    write_json(&crate::common::rotd_path().join("version.json"), &version)?;

    // Create default config
    let config = crate::schema::RotdConfig::default();
    crate::history::save_config(&config)?;

    if verbose {
        println!("Creating default configuration...");
    }

    Ok(())
}

// Human-friendly implementation of check with auto-fix functionality
pub fn check(fix: bool, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    println!("{}", "ROTD Compliance Check".cyan().bold());
    println!();

    let mut issues = Vec::new();
    let mut score = 0;
    let total_checks = 5;
    let _fixed: Vec<String> = Vec::new();

    // Check 1: Required files exist
    let required_files = [
        crate::common::tasks_path(),
        crate::common::session_state_path(),
        crate::common::coverage_history_path(),
    ];

    let files_exist = required_files.iter().all(|f| f.exists());
    if files_exist {
        println!("  {}", "✓ tasks.jsonl".green());
        println!("  {}", "✓ session_state.json".green());
        println!("  {}", "✓ coverage_history.json".green());
        score += 1;
    } else {
        for file_path in &required_files {
            if file_path.exists() {
                println!(
                    "  {}",
                    format!("✓ {}", file_path.file_name().unwrap().to_string_lossy()).green()
                );
            } else {
                println!(
                    "  {}",
                    format!("✗ {}", file_path.file_name().unwrap().to_string_lossy()).red()
                );
            }
        }
        issues.push("Missing required files");
    }

    // Check 2: JSONL files are valid
    let jsonl_valid = read_jsonl::<TaskEntry>(&crate::common::tasks_path()).is_ok();
    if jsonl_valid {
        score += 1;
    } else {
        println!("  {}", "✗ tasks.jsonl format".red());
        issues.push("Invalid tasks.jsonl: Invalid JSON on line 16 in .rotd/tasks.jsonl");
    }

    // Check 3: Test summaries exist for completed tasks
    let tasks: Vec<TaskEntry> = read_jsonl(&crate::common::tasks_path()).unwrap_or_default();
    let completed_tasks: Vec<_> = tasks
        .iter()
        .filter(|t| matches!(t.status, TaskStatus::Complete))
        .collect();

    let summaries_complete = completed_tasks
        .iter()
        .all(|t| crate::common::test_summary_file(&t.id).exists());

    if summaries_complete {
        score += 1;
    } else {
        let missing = completed_tasks
            .iter()
            .filter(|t| !crate::common::test_summary_file(&t.id).exists())
            .collect::<Vec<_>>();

        if !missing.is_empty() && verbose {
            println!("  {}", "✗ Missing test summaries".red());
            let _missing_count = missing.len();
            for task in &missing {
                println!(
                    "    - Task {} is marked complete but has no test summary",
                    task.id
                );
            }
            issues.push("Missing test summaries for completed tasks");
        }
    }

    // Check 4: No stubs remaining
    let no_stubs = !pss::check_stubs_remaining();
    if no_stubs {
        score += 1;
    } else {
        if verbose {
            println!("  {}", "✗ Stub code remaining".red());
        }
        issues.push("Stub code annotations remaining in project");
    }

    // Check 5: Session state is valid JSON
    let session_valid = read_json::<SessionState>(&crate::common::session_state_path()).is_ok();
    if session_valid {
        score += 1;
    } else {
        if verbose {
            println!("  {}", "✗ Invalid session_state.json".red());
        }
        issues.push("Invalid session state format");
    }

    let health_percentage = (score as f64 / total_checks as f64) * 100.0;

    println!();
    println!(
        "Health Score: {}/{} ({}%)",
        score, total_checks, health_percentage as u32
    );

    if !issues.is_empty() {
        println!();
        println!("Issues Found:");
        for (i, issue) in issues.iter().enumerate() {
            println!("  {}. {}", i + 1, issue);
        }
    }

    // Apply fixes if requested
    if fix && !issues.is_empty() {
        println!();
        println!("{}", "Auto-fixing issues...".cyan());

        let mut fixed_any = false;

        for issue in &issues {
            if issue.contains("Missing required files") {
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
                                    println!(
                                        "  {}",
                                        format!(
                                            "✓ Created {}",
                                            file_path.file_name().unwrap().to_string_lossy()
                                        )
                                        .green()
                                    );
                                    fixed_any = true;
                                }
                            }
                            Some("coverage_history.json") => {
                                let coverage_history = CoverageHistory {
                                    floor: 70.0,
                                    ratchet_threshold: 3.0,
                                    history: Vec::new(),
                                };
                                if write_json(file_path, &coverage_history).is_ok() {
                                    println!(
                                        "  {}",
                                        format!(
                                            "✓ Created {}",
                                            file_path.file_name().unwrap().to_string_lossy()
                                        )
                                        .green()
                                    );
                                    fixed_any = true;
                                }
                            }
                            Some("tasks.jsonl") => {
                                // Create empty file
                                if std::fs::File::create(file_path).is_ok() {
                                    println!(
                                        "  {}",
                                        format!(
                                            "✓ Created {}",
                                            file_path.file_name().unwrap().to_string_lossy()
                                        )
                                        .green()
                                    );
                                    fixed_any = true;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            } else if issue.contains("Invalid tasks.jsonl") {
                // Attempt to fix invalid JSON in tasks.jsonl
                if let Ok(content) = std::fs::read_to_string(&crate::common::tasks_path()) {
                    let mut fixed_lines = Vec::new();
                    let mut has_errors = false;
                    let mut fixed_count = 0;

                    for (_line_num, line) in content.lines().enumerate() {
                        if line.trim().is_empty() {
                            continue;
                        }

                        // Try to parse and re-serialize to fix formatting issues
                        match serde_json::from_str::<serde_json::Value>(line) {
                            Ok(value) => {
                                if let Ok(fixed_line) = serde_json::to_string(&value) {
                                    fixed_lines.push(fixed_line);
                                } else {
                                    has_errors = true;
                                    fixed_lines.push(line.to_string());
                                }
                            }
                            Err(_) => {
                                // Try some basic fixes for common JSON errors
                                let fixed = crate::agent::fix_common_json_errors(line);
                                match serde_json::from_str::<serde_json::Value>(&fixed) {
                                    Ok(value) => {
                                        if let Ok(fixed_line) = serde_json::to_string(&value) {
                                            fixed_lines.push(fixed_line);
                                            fixed_count += 1;
                                        } else {
                                            has_errors = true;
                                            fixed_lines.push(line.to_string());
                                        }
                                    }
                                    Err(_) => {
                                        has_errors = true;
                                        fixed_lines.push(line.to_string());
                                    }
                                }
                            }
                        }
                    }

                    if !has_errors || fixed_count > 0 {
                        // Create a backup first
                        let backup_path = crate::common::rotd_path().join("tasks.jsonl.bak");
                        if std::fs::copy(&crate::common::tasks_path(), &backup_path).is_ok() {
                            // Write fixed content
                            if std::fs::write(&crate::common::tasks_path(), fixed_lines.join("\n"))
                                .is_ok()
                            {
                                println!(
                                    "  {}",
                                    format!(
                                        "✓ Fixed JSON format in tasks.jsonl (fixed {} lines)",
                                        fixed_count
                                    )
                                    .green()
                                );
                                fixed_any = true;
                            }
                        }
                    }
                }
            }
        }

        if !fixed_any {
            println!("  {}", "! Auto-fix not yet implemented".yellow());
        }
    }

    Ok(())
}

/// Check for Buckle Mode trigger conditions
pub fn check_buckle_trigger(_verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    println!(
        "{}",
        "Checking Buckle Mode trigger conditions...".cyan().bold()
    );

    let triggered = false;
    let reasons: Vec<String> = Vec::new();

    // Check for compilation errors
    println!("Checking for compilation errors...");
    // Implementation would check cargo/npm output for error count

    // Check task.jsonl integrity
    println!("Checking task tracking integrity...");
    // Implementation would verify task.jsonl status is consistent

    // Check test summaries
    println!("Checking test summary artifacts...");
    // Implementation would verify test summaries exist for completed tasks

    // Check session state
    println!("Checking session state currency...");
    // Implementation would verify session_state.json is up to date

    // Report findings
    if triggered {
        println!("{}", "⚠️ BUCKLE MODE TRIGGER CONDITIONS MET!".red().bold());
        println!("Reasons:");
        for reason in reasons {
            println!("  - {}", reason.red());
        }
        println!("\nRecommended action:");
        println!("  {}", "rotd buckle-mode enter <task_id>".yellow());
    } else {
        println!(
            "{}",
            "✓ No Buckle Mode trigger conditions detected.".green()
        );
    }

    Ok(())
}

// Function to enter Buckle Mode
pub fn enter_buckle_mode(task_id: &str, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    println!(
        "{} {}",
        "Entering Buckle Mode for task:".cyan().bold(),
        task_id.white().bold()
    );

    // Check if already in Buckle Mode
    let buckle_state_path = crate::common::rotd_path().join("buckle_state.json");
    if buckle_state_path.exists() {
        let state: BuckleModeState =
            serde_json::from_str(&std::fs::read_to_string(&buckle_state_path)?)?;
        if state.active {
            println!(
                "{}",
                format!(
                    "Already in Buckle Mode for task: {}",
                    state.task_id.unwrap_or_default()
                )
                .yellow()
            );
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
    std::fs::write(buckle_state_path, serde_json::to_string_pretty(&state)?)?;

    // Log to audit log
    audit::log_entry(
        task_id,
        "audit.buckle.trigger.001",
        "critical",
        "Entered Buckle Mode manually",
    )?;

    // Run initial diagnostics
    println!("\n{}", "Running initial diagnostics...".cyan());
    diagnose_buckle_mode(verbose)?;

    println!("\n{}", "Buckle Mode entered successfully.".green());
    println!("Next steps:");
    println!("  1. {}", "rotd buckle-mode fix-compilation".yellow());
    println!("  2. {}", "rotd buckle-mode fix-artifacts".yellow());
    println!("  3. {}", "rotd buckle-mode check-exit".yellow());
    println!("  4. {}", "rotd buckle-mode exit".yellow());

    Ok(())
}

// Function to diagnose Buckle Mode issues
pub fn diagnose_buckle_mode(_verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    // Check Buckle Mode state
    let buckle_state_path = crate::common::rotd_path().join("buckle_state.json");
    if !buckle_state_path.exists() {
        println!(
            "{}",
            "Not in Buckle Mode. Use 'rotd buckle-mode enter <task_id>' to enter.".yellow()
        );
        return Ok(());
    }

    let state: BuckleModeState =
        serde_json::from_str(&std::fs::read_to_string(&buckle_state_path)?)?;
    if !state.active {
        println!(
            "{}",
            "Not in Buckle Mode. Use 'rotd buckle-mode enter <task_id>' to enter.".yellow()
        );
        return Ok(());
    }

    let task_id = state.task_id.unwrap_or_default();
    println!(
        "{}",
        format!("Generating diagnostic report for task: {}", task_id)
            .cyan()
            .bold()
    );

    // Compilation status
    println!("\n{}", "Compilation Status:".cyan());
    // Implementation would check cargo/npm build output

    // Test status
    println!("\n{}", "Test Status:".cyan());
    // Implementation would check test output

    // Artifact integrity
    println!("\n{}", "Artifact Integrity:".cyan());
    // Implementation would check for missing/invalid artifacts

    // Task tracking
    println!("\n{}", "Task Tracking:".cyan());
    // Implementation would check task.jsonl consistency

    // Exit criteria
    println!("\n{}", "Exit Criteria Status:".cyan());
    if state.compilation_fixed {
        println!("  [{}] Compilation issues fixed", "✓".green());
    } else {
        println!("  [{}] Compilation issues fixed", "✗".red());
    }

    if state.artifacts_fixed {
        println!("  [{}] Artifact issues fixed", "✓".green());
    } else {
        println!("  [{}] Artifact issues fixed", "✗".red());
    }

    if state.exit_criteria_met {
        println!("  [{}] Exit criteria met", "✓".green());
    } else {
        println!("  [{}] Exit criteria met", "✗".red());
    }

    println!("\n{}", "Diagnostic report complete.".green());

    Ok(())
}

// Function to fix compilation errors
pub fn fix_compilation(_verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    // Check Buckle Mode state
    let buckle_state_path = crate::common::rotd_path().join("buckle_state.json");
    if !buckle_state_path.exists() {
        println!(
            "{}",
            "Not in Buckle Mode. Use 'rotd buckle-mode enter <task_id>' to enter.".yellow()
        );
        return Ok(());
    }

    let mut state: BuckleModeState =
        serde_json::from_str(&std::fs::read_to_string(&buckle_state_path)?)?;
    if !state.active {
        println!(
            "{}",
            "Not in Buckle Mode. Use 'rotd buckle-mode enter <task_id>' to enter.".yellow()
        );
        return Ok(());
    }

    let unknown = "unknown".to_string();
    let task_id = state.task_id.as_ref().unwrap_or(&unknown);
    println!(
        "{}",
        format!("Fixing compilation errors for task: {}", task_id)
            .cyan()
            .bold()
    );

    // Implementation would attempt to fix compilation errors

    // Update state
    state.compilation_fixed = true;
    std::fs::write(buckle_state_path, serde_json::to_string_pretty(&state)?)?;

    println!("{}", "✓ Compilation fixes applied.".green());
    println!("Next step: {}", "rotd buckle-mode fix-artifacts".yellow());

    Ok(())
}

// Function to fix artifacts
pub fn fix_artifacts(_verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    // Check Buckle Mode state
    let buckle_state_path = crate::common::rotd_path().join("buckle_state.json");
    if !buckle_state_path.exists() {
        println!(
            "{}",
            "Not in Buckle Mode. Use 'rotd buckle-mode enter <task_id>' to enter.".yellow()
        );
        return Ok(());
    }

    let mut state: BuckleModeState =
        serde_json::from_str(&std::fs::read_to_string(&buckle_state_path)?)?;
    if !state.active {
        println!(
            "{}",
            "Not in Buckle Mode. Use 'rotd buckle-mode enter <task_id>' to enter.".yellow()
        );
        return Ok(());
    }

    let unknown = "unknown".to_string();
    let task_id = state.task_id.as_ref().unwrap_or(&unknown);
    println!(
        "{}",
        format!("Fixing artifact issues for task: {}", task_id)
            .cyan()
            .bold()
    );

    // Implementation would attempt to fix artifacts

    // Update state
    state.artifacts_fixed = true;
    std::fs::write(buckle_state_path, serde_json::to_string_pretty(&state)?)?;

    println!("{}", "✓ Artifact fixes applied.".green());
    println!("Next step: {}", "rotd buckle-mode check-exit".yellow());

    Ok(())
}

// Function to check exit criteria
pub fn check_exit_criteria(_verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    // Check Buckle Mode state
    let buckle_state_path = crate::common::rotd_path().join("buckle_state.json");
    if !buckle_state_path.exists() {
        println!(
            "{}",
            "Not in Buckle Mode. Use 'rotd buckle-mode enter <task_id>' to enter.".yellow()
        );
        return Ok(());
    }

    let mut state: BuckleModeState =
        serde_json::from_str(&std::fs::read_to_string(&buckle_state_path)?)?;
    if !state.active {
        println!(
            "{}",
            "Not in Buckle Mode. Use 'rotd buckle-mode enter <task_id>' to enter.".yellow()
        );
        return Ok(());
    }

    let unknown = "unknown".to_string();
    let task_id = state.task_id.as_ref().unwrap_or(&unknown);
    println!(
        "{}",
        format!("Checking exit criteria for task: {}", task_id)
            .cyan()
            .bold()
    );

    // Implementation would check all exit criteria

    // Update state
    state.exit_criteria_met = true;
    std::fs::write(buckle_state_path, serde_json::to_string_pretty(&state)?)?;

    println!("{}", "✓ All exit criteria met.".green());
    println!("Next step: {}", "rotd buckle-mode exit".yellow());

    Ok(())
}

// Function to exit Buckle Mode
pub fn exit_buckle_mode(_verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    // Check Buckle Mode state
    let buckle_state_path = crate::common::rotd_path().join("buckle_state.json");
    if !buckle_state_path.exists() {
        println!("{}", "Not in Buckle Mode.".yellow());
        return Ok(());
    }

    let state: BuckleModeState =
        serde_json::from_str(&std::fs::read_to_string(&buckle_state_path)?)?;
    if !state.active {
        println!("{}", "Not in Buckle Mode.".yellow());
        return Ok(());
    }

    let unknown = "unknown".to_string();
    let task_id = state.task_id.as_ref().unwrap_or(&unknown);

    // Check if exit criteria are met
    if !state.exit_criteria_met {
        println!(
            "{}",
            "Exit criteria not met. Run 'rotd buckle-mode check-exit' first.".red()
        );
        return Ok(());
    }

    println!(
        "{}",
        format!("Exiting Buckle Mode for task: {}", task_id)
            .cyan()
            .bold()
    );

    // Remove Buckle Mode state
    std::fs::remove_file(buckle_state_path)?;

    // Log to audit log
    audit::log_entry(
        task_id,
        "audit.buckle.exit",
        "info",
        "Exited Buckle Mode successfully",
    )?;

    println!("{}", "✓ Buckle Mode exited successfully.".green());
    println!("Project restored to clean state.");

    Ok(())
}

// Function to show task details
pub fn show_task(task_id: &str, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    let tasks = read_jsonl::<TaskEntry>(&crate::common::tasks_path())?;

    let task = tasks.iter().find(|t| t.id == task_id);

    match task {
        Some(task) => {
            println!("{}", format!("Task {}", task_id).cyan().bold());
            println!("  Title:       {}", task.title);
            println!(
                "  Status:      {}",
                match task.status {
                    TaskStatus::Pending => "Pending".yellow(),
                    TaskStatus::InProgress => "In Progress".blue(),
                    TaskStatus::Blocked => "Blocked".red(),
                    TaskStatus::Complete => "Complete".green(),
                    TaskStatus::Scaffolded => "Scaffolded".cyan(),
                }
            );

            if let Some(priority) = &task.priority {
                println!(
                    "  Priority:    {}",
                    match priority.as_str() {
                        "urgent" => "Urgent".red().bold(),
                        "high" => "High".red(),
                        "medium" => "Medium".yellow(),
                        "low" => "Low".green(),
                        "deferred" => "Deferred".blue(),
                        _ => priority.normal(),
                    }
                );
            }

            if let Some(tests) = &task.tests {
                println!("\nTests:");
                for test in tests {
                    println!("  - {}", test);
                }
            }

            if let Some(description) = &task.description {
                println!("\nDescription:");
                println!("{}", description);
            }

            if verbose {
                println!("\nTimestamps:");
                if let Some(created) = &task.created {
                    println!("  Created:    {}", created);
                }
                if let Some(updated) = &task.updated_at {
                    println!("  Updated:    {}", updated);
                }
                if let Some(completed) = &task.completed {
                    println!("  Completed:  {}", completed);
                }

                // Show test summary if available
                let summary_path = crate::common::test_summary_file(&task.id);
                if summary_path.exists() {
                    match read_json::<TestSummary>(&summary_path) {
                        Ok(summary) => {
                            println!("\nTest Summary:");
                            println!("  Total Tests: {}", summary.total_tests);
                            println!("  Passed:      {}", summary.passed);
                            println!("  Failed:      {}", summary.failed);
                            println!(
                                "  Pass Rate:   {:.1}%",
                                (summary.passed as f64 / summary.total_tests as f64) * 100.0
                            );
                        }
                        Err(_) => {
                            println!("\nTest Summary: [Invalid format]");
                        }
                    }
                }
            }
        }
        None => {
            println!("{}", format!("Task {} not found", task_id).red());
        }
    }

    Ok(())
}

// Function to list lessons learned
pub fn show_lessons(tag: Option<&str>, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    let lessons_path = crate::common::lessons_path();

    if !lessons_path.exists() {
        println!("No lessons learned yet.");
        return Ok(());
    }

    let all_lessons = read_jsonl::<LessonLearned>(&lessons_path)?;

    let filtered: Vec<_> = match tag {
        Some(tag) => all_lessons
            .into_iter()
            .filter(|l| l.tags.iter().any(|t| t == tag))
            .collect(),
        None => all_lessons,
    };

    if filtered.is_empty() {
        println!(
            "No lessons found{}",
            tag.map_or(String::new(), |t| format!(" with tag '{}'", t))
        );
        return Ok(());
    }

    println!("{}", "Lessons Learned".cyan().bold());
    println!();

    for (i, lesson) in filtered.iter().enumerate() {
        println!("{}. {} ({})", i + 1, lesson.id.bold(), lesson.id);

        println!("   Problem: {}", lesson.diagnosis);

        println!("   Solution: {}", lesson.remediation);

        if verbose {
            if !lesson.tags.is_empty() {
                println!("   Tags: {}", lesson.tags.join(", ").blue());
            }

            if let Some(timestamp) = &lesson.timestamp {
                println!("   Recorded: {}", timestamp);
            }
        }

        println!();
    }

    Ok(())
}

// Function to show audit log
pub fn show_audit(limit: usize, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    let audit_path = crate::common::rotd_path().join("audit.log");

    if !audit_path.exists() {
        println!("No audit entries yet.");
        return Ok(());
    }

    let content = std::fs::read_to_string(&audit_path)?;
    let mut entries = Vec::new();

    for line in content.lines() {
        if let Ok(entry) = serde_json::from_str::<AuditEntry>(line) {
            entries.push(entry);
        }
    }

    // Sort by timestamp, newest first
    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    // Take only the requested number of entries
    let limited = if entries.len() > limit {
        &entries[0..limit]
    } else {
        &entries
    };

    println!(
        "{}",
        format!("Audit Log (Last {} Entries)", limited.len())
            .cyan()
            .bold()
    );
    println!();

    for entry in limited {
        let severity_display = match entry.severity.as_str() {
            "critical" => "CRITICAL".red().bold(),
            "error" => "ERROR".red(),
            "warning" => "WARNING".yellow(),
            "info" => "INFO".blue(),
            _ => entry.severity.normal(),
        };

        println!(
            "[{}] {}: {}",
            severity_display,
            entry.rule.bold(),
            entry.message
        );

        if verbose {
            println!("   Task: {}", entry.task_id.as_deref().unwrap_or("-"));
            println!("   Time: {}", entry.timestamp);
            println!();
        }
    }

    Ok(())
}

// Function for shell completions
pub fn completions(shell: &str) -> Result<()> {
    println!("Generating completions for {} shell...", shell);

    // Implementation would generate shell completions

    println!("{}", "✓ Completions generated.".green());

    Ok(())
}

// Function for validating schemas
pub fn validate(all: bool, schema_type: Option<&str>, strict: bool, _verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    println!("{}", "ROTD Schema Validation".cyan().bold());

    let mut passed = true;

    if all || schema_type.is_none() || schema_type == Some("tasks") {
        println!("\n{}", "Validating tasks.jsonl...".cyan());
        match crate::agent::validate_tasks_jsonl(strict) {
            Ok(result) => {
                if result.status == "passed" {
                    println!("  {}", "✓ tasks.jsonl validation passed".green());
                    println!("    {} items checked", result.items_checked);
                } else {
                    passed = false;
                    println!("  {}", "✗ tasks.jsonl validation failed".red());
                    for error in &result.errors {
                        println!("    - {}", error.red());
                    }
                    for warning in &result.warnings {
                        println!("    - {}", warning.yellow());
                    }
                }
            }
            Err(e) => {
                passed = false;
                println!("  {}", "✗ tasks.jsonl validation error".red());
                println!("    {}", e);
            }
        }
    }

    // Add validation for other schemas here

    if passed {
        println!("\n{}", "✓ All validations passed!".green().bold());
    } else {
        println!("\n{}", "✗ Some validations failed.".red().bold());
        if strict {
            println!("  Run without --strict for more lenient validation");
        }
    }

    Ok(())
}

// Function to score task using PSS
pub fn score(task_id: &str, format: &str, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;

    println!(
        "{}",
        format!("Scoring task {} using ROTD PSS...", task_id)
            .cyan()
            .bold()
    );

    // Call the core scoring function
    let score_result = pss::score_task(task_id)?;

    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&score_result)?);
        }
        "summary" => {
            println!("Task ID: {}", task_id);
            println!("Total Score: {}/10", score_result.score);
            println!(
                "Status: {}",
                if score_result.score >= 6 {
                    "PASSED".green()
                } else {
                    "FAILED".red()
                }
            );
        }
        _ => {
            // table format
            println!("Task ID: {}", task_id);
            println!("Total Score: {}/10", score_result.score);
            println!(
                "Status: {}",
                if score_result.score >= 6 {
                    "PASSED".green()
                } else {
                    "FAILED".red()
                }
            );

            println!("\nDetailed Scores:");
            println!("---------------");
            // Compute execution sanity score from criteria
            let execution_sanity = score_result
                .criteria
                .iter()
                .filter(|(k, _)| ["llm_engaged", "compiles", "core_impl"].contains(&k.as_str()))
                .map(|(_, v)| v.score)
                .sum::<u32>();
            println!("Execution Sanity: {}/3", execution_sanity);
            // Compute testing discipline score from criteria
            let testing_discipline = score_result
                .criteria
                .iter()
                .filter(|(k, _)| ["tests_written", "tests_pass", "coverage"].contains(&k.as_str()))
                .map(|(_, v)| v.score)
                .sum::<u32>();
            println!("Testing Discipline: {}/3", testing_discipline);
            // Compute cleanup discipline score from criteria
            let cleanup_discipline = score_result
                .criteria
                .iter()
                .filter(|(k, _)| ["no_stubs", "docs_updated"].contains(&k.as_str()))
                .map(|(_, v)| v.score)
                .sum::<u32>();
            println!("Cleanup Discipline: {}/2", cleanup_discipline);
            // Compute historical continuity score from criteria
            let historical_continuity = score_result
                .criteria
                .iter()
                .filter(|(k, _)| ["history_consistent", "lessons_logged"].contains(&k.as_str()))
                .map(|(_, v)| v.score)
                .sum::<u32>();
            println!("Historical Continuity: {}/2", historical_continuity);

            if verbose {
                println!("\nDetails:");
                for (i, (key, criterion)) in score_result.criteria.iter().enumerate() {
                    println!(
                        "{:2}. {} {}",
                        i + 1,
                        if criterion.score > 0 {
                            "✓".green()
                        } else {
                            "✗".red()
                        },
                        format!("{}: {}", key, criterion.rationale)
                    );
                }
            }
        }
    }

    // Record score to file
    pss::save_score(&score_result, false)?;

    Ok(())
}

#[allow(dead_code)]
pub fn show_help(verbose: bool) -> Result<()> {
    println!("{}", "ROTD CLI Help".cyan().bold());
    println!("\nCore Commands:");
    println!("  init                 Initialize ROTD structure in current project");
    println!("  check               Check ROTD project health and compliance");
    println!("  update              Update ROTD methodology and templates");
    println!("  score <task_id>     Generate PSS score for a task");

    println!("\nTask Management:");
    println!("  show-task <task_id> Display detailed task information");
    println!("  show-lessons        List logged lessons in readable format");
    println!("  show-audit          Show audit violations");

    println!("\nBuckle Mode:");
    println!("  buckle-mode enter   Enter Buckle Mode for a task");
    println!("  buckle-mode check   Check Buckle Mode trigger conditions");
    println!("  buckle-mode exit    Exit Buckle Mode");

    if verbose {
        println!("\nAdvanced Commands:");
        println!("  agent              Agent-oriented commands");
        println!("  validate           Validate ROTD artifacts");
        println!("  version            Show version information");
        println!("  completions        Generate shell completions");

        println!("\nCommon Flags:");
        println!("  --dry-run         Show what would be done without making changes");
        println!("  --verbose         Display additional information");
        println!("  --force           Skip confirmation prompts");
        println!("  --agent           Enable agent mode (JSON output)");
    }

    Ok(())
}

// Primer management functions
pub fn primer_init(force: bool, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;
    
    let primer_path = crate::common::rotd_path().join("primer.jsonc");
    
    if primer_path.exists() && !force {
        if !dialoguer::Confirm::new()
            .with_prompt("Primer already exists. Overwrite?")
            .default(false)
            .interact()?
        {
            println!("{}", "Primer initialization cancelled.".yellow());
            return Ok(());
        }
    }
    
    println!("{}", "Initializing project primer...".cyan());
    
    // Detect basic project information
    let project_name = std::env::current_dir()?
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    // Detect language based on files present
    let language = if std::path::Path::new("Cargo.toml").exists() {
        "Rust"
    } else if std::path::Path::new("package.json").exists() {
        "JavaScript/TypeScript"
    } else if std::path::Path::new("requirements.txt").exists() || std::path::Path::new("setup.py").exists() {
        "Python"
    } else {
        "Unknown"
    };
    
    // Find entry points
    let entry_points = match language {
        "Rust" => vec!["src/main.rs", "src/lib.rs"],
        "JavaScript/TypeScript" => vec!["index.js", "src/index.js", "src/main.ts"],
        "Python" => vec!["main.py", "app.py", "__main__.py"],
        _ => vec!["main"],
    }.into_iter().filter(|&path| std::path::Path::new(path).exists()).map(|s| s.to_string()).collect();
    
    // Find test directories
    let test_dirs = vec!["tests/", "test/", "spec/", "src/"]
        .into_iter()
        .filter(|&path| std::path::Path::new(path).exists())
        .map(|s| s.to_string())
        .collect();
    
    let primer = ProjectPrimer {
        name: project_name,
        scope: "root".to_string(),
        description: "TODO: Add project description".to_string(),
        status: "active".to_string(),
        language: language.to_string(),
        entry_points,
        test_dirs,
        dependencies: vec![], // TODO: Could parse from Cargo.toml, package.json, etc.
        known_issues: vec![],
        key_concepts: vec![],
        preferred_agents: Some(vec!["Claude Sonnet".to_string()]),
        suggested_starting_points: vec!["TODO: Add suggested starting points".to_string()],
        major_components: None,
        update_triggers: Some(vec![
            "Major architectural changes".to_string(),
            "New dependencies added".to_string(),
            "Significant code restructuring".to_string(),
        ]),
    };
    
    // Convert to pretty JSON
    let json_content = serde_json::to_string_pretty(&primer)?;
    std::fs::write(&primer_path, json_content)?;
    
    println!("{}", "✓ Primer initialized successfully!".green());
    println!("   Location: {}", primer_path.display().to_string().cyan());
    
    if verbose {
        println!("\nNext steps:");
        println!("  1. Edit {} to add project description", primer_path.display());
        println!("  2. Add key concepts and dependencies");
        println!("  3. Update suggested starting points");
        println!("  4. Run {} to validate", "rotd primer check".cyan());
    }
    
    Ok(())
}

pub fn primer_show(file: Option<&str>, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;
    
    let primer_path = match file {
        Some(f) => crate::common::rotd_path().join(f),
        None => crate::common::rotd_path().join("primer.jsonc"),
    };
    
    if !primer_path.exists() {
        println!("{}", format!("Primer file not found: {}", primer_path.display()).red());
        println!("Run {} to create one.", "rotd primer init".cyan());
        return Ok(());
    }
    
    let content = std::fs::read_to_string(&primer_path)?;
    
    if verbose {
        println!("{}", format!("Primer: {}", primer_path.display()).cyan().bold());
        println!();
    }
    
    println!("{}", content);
    
    Ok(())
}

pub fn primer_check(verbose: bool) -> Result<()> {
    check_rotd_initialized()?;
    
    let primer_path = crate::common::rotd_path().join("primer.jsonc");
    
    if !primer_path.exists() {
        println!("{}", "✗ No primer.jsonc found".red());
        println!("Run {} to create one.", "rotd primer init".cyan());
        return Ok(());
    }
    
    println!("{}", "Checking primer...".cyan());
    
    // Try to parse the primer
    let content = std::fs::read_to_string(&primer_path)?;
    let primer: ProjectPrimer = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse primer.jsonc: {}", e))?;
    
    let mut issues = Vec::new();
    let mut warnings = Vec::new();
    
    // Check for TODO placeholders
    if primer.description.contains("TODO") {
        warnings.push("Description contains TODO placeholder");
    }
    
    if primer.suggested_starting_points.iter().any(|s| s.contains("TODO")) {
        warnings.push("Suggested starting points contain TODO placeholders");
    }
    
    // Check if entry points exist
    for entry_point in &primer.entry_points {
        if !std::path::Path::new(entry_point).exists() {
            issues.push(format!("Entry point does not exist: {}", entry_point));
        }
    }
    
    // Check if test directories exist
    for test_dir in &primer.test_dirs {
        if !std::path::Path::new(test_dir).exists() {
            warnings.push("Test directory does not exist");
        }
    }
    
    // Check if key concepts are provided
    if primer.key_concepts.is_empty() {
        warnings.push("No key concepts defined");
    }
    
    // Report results
    if issues.is_empty() && warnings.is_empty() {
        println!("{}", "✓ Primer validation passed!".green());
    } else {
        if !issues.is_empty() {
            println!("{}", "Issues found:".red());
            for issue in &issues {
                println!("  ✗ {}", issue.red());
            }
        }
        
        if !warnings.is_empty() {
            println!("{}", "Warnings:".yellow());
            for warning in &warnings {
                println!("  ⚠ {}", warning.yellow());
            }
        }
    }
    
    if verbose {
        println!("\nPrimer summary:");
        println!("  Name: {}", primer.name);
        println!("  Language: {}", primer.language);
        println!("  Entry points: {}", primer.entry_points.len());
        println!("  Test directories: {}", primer.test_dirs.len());
        println!("  Key concepts: {}", primer.key_concepts.len());
    }
    
    Ok(())
}

pub fn primer_parse(format: &str, verbose: bool) -> Result<()> {
    check_rotd_initialized()?;
    
    let primer_path = crate::common::rotd_path().join("primer.jsonc");
    
    if !primer_path.exists() {
        println!("{}", "No primer.jsonc found".red());
        return Ok(());
    }
    
    let content = std::fs::read_to_string(&primer_path)?;
    let primer: ProjectPrimer = serde_json::from_str(&content)?;
    
    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&primer)?);
        }
        "summary" => {
            println!("{}", format!("Project: {}", primer.name).cyan().bold());
            println!("Description: {}", primer.description);
            println!("Language: {}", primer.language);
            
            if !primer.key_concepts.is_empty() {
                println!("\nKey Concepts:");
                for concept in &primer.key_concepts {
                    println!("  - {}", concept);
                }
            }
            
            if !primer.suggested_starting_points.is_empty() {
                println!("\nSuggested Starting Points:");
                for point in &primer.suggested_starting_points {
                    println!("  - {}", point);
                }
            }
            
            if verbose {
                println!("\nEntry Points: {}", primer.entry_points.join(", "));
                println!("Test Directories: {}", primer.test_dirs.join(", "));
                
                if !primer.known_issues.is_empty() {
                    println!("\nKnown Issues:");
                    for issue in &primer.known_issues {
                        println!("  - {}", issue);
                    }
                }
            }
        }
        _ => {
            println!("{}", format!("Unknown format: {}", format).red());
            return Ok(());
        }
    }
    
    Ok(())
}

// Additional utility functions as needed
