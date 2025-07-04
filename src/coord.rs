use anyhow::Result;
use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

use crate::fs_ops::{read_json, with_lock, with_lock_result, write_json};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkRegistryTask {
    pub id: String,
    pub title: String,
    pub status: WorkStatus,
    pub priority: TaskPriority,
    pub claimed_by: Option<String>,
    pub claimed_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub blocked_reason: Option<String>,
    pub reviewer_id: Option<String>,
    pub capability: Option<String>,
    pub skill_level: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WorkStatus {
    Unclaimed,
    Claimed,
    Blocked,
    Review,
    Done,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    Urgent,
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkRegistry {
    pub tasks: Vec<WorkRegistryTask>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyMap {
    #[serde(flatten)]
    pub deps: std::collections::HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaTracker {
    pub tokens_used: u64,
    pub last_reset: DateTime<Utc>,
    pub requests: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockMetadata {
    pub holder: String,
    pub since: DateTime<Utc>,
}

pub fn get_agent_id() -> Result<String> {
    // Try to get from environment first
    if let Ok(id) = std::env::var("ROTD_AGENT_ID") {
        return Ok(id);
    }

    // Otherwise generate and store
    let id = Uuid::new_v4().to_string();
    Ok(id)
}

pub fn touch_heartbeat(agent_id: &str) -> Result<()> {
    let heartbeat_path =
        PathBuf::from(".rotd/coordination/heartbeat").join(format!("{}.beat", agent_id));

    // Create parent directory if it doesn't exist
    if let Some(parent) = heartbeat_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Touch the file
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(&heartbeat_path)?;

    Ok(())
}

pub fn check_heartbeat(agent_id: &str) -> Result<Option<std::time::SystemTime>> {
    let heartbeat_path =
        PathBuf::from(".rotd/coordination/heartbeat").join(format!("{}.beat", agent_id));

    if heartbeat_path.exists() {
        let metadata = fs::metadata(&heartbeat_path)?;
        Ok(Some(metadata.modified()?))
    } else {
        Ok(None)
    }
}

pub fn clean_stale_locks(timeout_secs: u64) -> Result<Vec<String>> {
    let mut cleaned = Vec::new();
    let lock_dir = PathBuf::from(".rotd/coordination/agent_locks");

    if !lock_dir.exists() {
        return Ok(cleaned);
    }

    let now = std::time::SystemTime::now();

    for entry in fs::read_dir(&lock_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("lock") {
            // Parse lock filename: <task_id>.<agent_id>.lock
            if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                if let Some(agent_id) = filename.split('.').nth(1) {
                    // Check heartbeat
                    if let Some(last_beat) = check_heartbeat(agent_id)? {
                        if let Ok(elapsed) = now.duration_since(last_beat) {
                            if elapsed.as_secs() > timeout_secs {
                                // Stale lock, remove it
                                fs::remove_file(&path)?;
                                cleaned.push(filename.to_string());

                                // Update registry
                                let registry_path =
                                    PathBuf::from(".rotd/coordination/active_work_registry.json");
                                let lock_path =
                                    PathBuf::from(".rotd/coordination/.lock/registry.lock");

                                with_lock(&lock_path, || {
                                    let mut registry: WorkRegistry = read_json(&registry_path)?;

                                    // Find task and reset to unclaimed
                                    for task in &mut registry.tasks {
                                        if task.claimed_by.as_ref() == Some(&agent_id.to_string()) {
                                            task.status = WorkStatus::Unclaimed;
                                            task.claimed_by = None;
                                            task.claimed_at = None;
                                        }
                                    }

                                    write_json(&registry_path, &registry)?;
                                    Ok(())
                                })?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(cleaned)
}

pub fn append_coordination_log(message: &str) -> Result<()> {
    let log_path = PathBuf::from(".rotd/coordination/coordination.log");
    let lock_path = PathBuf::from(".rotd/coordination/.lock/coordination.lock");

    with_lock(&lock_path, || {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;

        writeln!(file, "[{}] {}", Utc::now().to_rfc3339(), message)?;
        Ok(())
    })
}

pub fn rotate_coordination_log() -> Result<()> {
    let log_path = PathBuf::from(".rotd/coordination/coordination.log");

    if log_path.exists() {
        let today = Utc::now().format("%Y-%m-%d");
        let archive_path =
            PathBuf::from(".rotd/coordination").join(format!("coordination-{}.log", today));

        fs::rename(&log_path, &archive_path)?;
    }

    Ok(())
}

use crate::CoordCommands;

pub fn handle_command(cmd: CoordCommands, is_agent_mode: bool, verbose: bool) -> Result<()> {
    match cmd {
        CoordCommands::Claim {
            capability,
            skill_level,
            any,
        } => cmd_claim(capability, skill_level, any, is_agent_mode),
        CoordCommands::Release { task_id } => cmd_release(&task_id, is_agent_mode),
        CoordCommands::Approve { task_id } => cmd_approve(&task_id, is_agent_mode),
        CoordCommands::Msg { message } => cmd_msg(&message, is_agent_mode),
        CoordCommands::Beat => cmd_beat(is_agent_mode),
        CoordCommands::CleanStale { timeout } => cmd_clean_stale(timeout, is_agent_mode),
        CoordCommands::Quota { add } => cmd_quota(add, is_agent_mode),
        CoordCommands::Ls => cmd_ls(is_agent_mode, verbose),
    }
}

fn cmd_claim(
    capability: Option<String>,
    skill_level: Option<String>,
    any: bool,
    is_agent_mode: bool,
) -> Result<()> {
    let agent_id = get_agent_id()?;
    let registry_path = PathBuf::from(".rotd/coordination/active_work_registry.json");
    let lock_dir = PathBuf::from(".rotd/coordination/.lock");
    fs::create_dir_all(&lock_dir)?;
    let lock_path = lock_dir.join("registry.lock");
    let deps_path = PathBuf::from(".rotd/coordination/dependency_map.json");

    let result = with_lock_result(&lock_path, || -> Result<Option<WorkRegistryTask>> {
        let mut registry: WorkRegistry = read_json(&registry_path)?;
        let deps: DependencyMap = if deps_path.exists() {
            read_json(&deps_path)?
        } else {
            DependencyMap {
                deps: std::collections::HashMap::new(),
            }
        };

        // Find first unclaimed task matching filters
        let mut claimed_task = None;

        // Sort tasks by priority if not using --any
        if !any {
            registry
                .tasks
                .sort_by(|a, b| match (&a.priority, &b.priority) {
                    (TaskPriority::Urgent, TaskPriority::Urgent) => std::cmp::Ordering::Equal,
                    (TaskPriority::Urgent, _) => std::cmp::Ordering::Less,
                    (_, TaskPriority::Urgent) => std::cmp::Ordering::Greater,
                    (TaskPriority::High, TaskPriority::High) => std::cmp::Ordering::Equal,
                    (TaskPriority::High, _) => std::cmp::Ordering::Less,
                    (_, TaskPriority::High) => std::cmp::Ordering::Greater,
                    (TaskPriority::Medium, TaskPriority::Low) => std::cmp::Ordering::Less,
                    (TaskPriority::Low, TaskPriority::Medium) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                });
        }

        // Create a list of task statuses to avoid borrowing issues
        let task_statuses: Vec<(String, WorkStatus)> = registry
            .tasks
            .iter()
            .map(|t| (t.id.clone(), t.status.clone()))
            .collect();

        for task in &mut registry.tasks {
            if task.status != WorkStatus::Unclaimed {
                continue;
            }

            // Check capability filter
            if let Some(ref cap) = capability {
                if task.capability.as_ref() != Some(cap) {
                    continue;
                }
            }

            // Check skill level filter
            if let Some(ref _skill) = skill_level {
                // TODO: Implement skill level comparison logic
            }

            // Check dependencies
            if let Some(task_deps) = deps.deps.get(&task.id) {
                let all_deps_done = task_deps.iter().all(|dep_id| {
                    task_statuses
                        .iter()
                        .any(|(id, status)| id == dep_id && *status == WorkStatus::Done)
                });

                if !all_deps_done {
                    continue; // Skip tasks with incomplete dependencies
                }
            }

            // Check if task has no existing lock
            let lock_dir = PathBuf::from(".rotd/coordination/agent_locks");
            fs::create_dir_all(&lock_dir)?;
            let lock_file = lock_dir.join(format!("{}.{}.lock", task.id, agent_id));

            if !lock_file.exists() {
                // Try to create lock atomically
                if let Ok(file) = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&lock_file)
                {
                    // Write lock metadata
                    let metadata = LockMetadata {
                        holder: agent_id.clone(),
                        since: Utc::now(),
                    };
                    serde_json::to_writer(&file, &metadata)?;

                    // Update task status
                    task.status = WorkStatus::Claimed;
                    task.claimed_by = Some(agent_id.clone());
                    task.claimed_at = Some(Utc::now());

                    claimed_task = Some(task.clone());
                    break;
                }
            }
        }

        if claimed_task.is_some() {
            write_json(&registry_path, &registry)?;
        }

        Ok(claimed_task)
    })?;

    if is_agent_mode {
        if let Some(ref task) = result {
            println!("{}", serde_json::to_string(&task)?);
        } else {
            println!("{}", r#"{"status":"no_eligible_task"}"#);
        }
    } else {
        if let Some(ref task) = result {
            println!("Claimed task {}: {}", task.id, task.title);
        } else {
            println!("No eligible tasks available");
        }
    }

    // Log the claim
    if let Some(ref task) = result {
        let msg = format!("{} ▶ claimed task {}", agent_id, task.id);
        append_coordination_log(&msg)?;
    }

    Ok(())
}

fn cmd_release(task_id: &str, is_agent_mode: bool) -> Result<()> {
    let agent_id = get_agent_id()?;
    let registry_path = PathBuf::from(".rotd/coordination/active_work_registry.json");
    let lock_path = PathBuf::from(".rotd/coordination/.lock/registry.lock");

    with_lock(&lock_path, || {
        let mut registry: WorkRegistry = read_json(&registry_path)?;

        // Find and update task
        let mut found = false;
        for task in &mut registry.tasks {
            if task.id == task_id && task.claimed_by.as_ref() == Some(&agent_id) {
                task.status = WorkStatus::Done;
                task.completed_at = Some(Utc::now());
                found = true;
                break;
            }
        }

        if !found {
            return Err(anyhow::anyhow!(
                "Task not found or not claimed by this agent"
            ));
        }

        write_json(&registry_path, &registry)?;

        // Remove lock file
        let lock_file = PathBuf::from(".rotd/coordination/agent_locks")
            .join(format!("{}.{}.lock", task_id, agent_id));
        if lock_file.exists() {
            fs::remove_file(&lock_file)?;
        }

        Ok(())
    })?;

    // Log the release
    let msg = format!("{} ▶ completed task {}", agent_id, task_id);
    append_coordination_log(&msg)?;

    if is_agent_mode {
        println!(
            "{}",
            serde_json::json!({
                "status": "success",
                "action": "release",
                "task_id": task_id
            })
        );
    } else {
        println!("Released task {}", task_id);
    }

    Ok(())
}

fn cmd_approve(task_id: &str, is_agent_mode: bool) -> Result<()> {
    let agent_id = get_agent_id()?;
    let registry_path = PathBuf::from(".rotd/coordination/active_work_registry.json");
    let lock_path = PathBuf::from(".rotd/coordination/.lock/registry.lock");

    with_lock(&lock_path, || {
        let mut registry: WorkRegistry = read_json(&registry_path)?;

        // Find and approve task
        let mut found = false;
        for task in &mut registry.tasks {
            if task.id == task_id && task.status == WorkStatus::Review {
                task.status = WorkStatus::Done;
                task.reviewer_id = Some(agent_id.clone());
                task.completed_at = Some(Utc::now());
                found = true;
                break;
            }
        }

        if !found {
            return Err(anyhow::anyhow!("Task not found or not in review status"));
        }

        write_json(&registry_path, &registry)?;
        Ok(())
    })?;

    if is_agent_mode {
        println!(
            "{}",
            serde_json::json!({
                "status": "success",
                "action": "approve",
                "task_id": task_id
            })
        );
    } else {
        println!("Approved task {}", task_id);
    }

    Ok(())
}

fn cmd_msg(message: &str, is_agent_mode: bool) -> Result<()> {
    let agent_id = get_agent_id()?;
    let full_msg = format!("{} ▶ {}", agent_id, message);
    append_coordination_log(&full_msg)?;

    if is_agent_mode {
        println!("{}", r#"{"status":"success","action":"msg"}"#);
    } else {
        println!("Message logged");
    }

    Ok(())
}

fn cmd_beat(is_agent_mode: bool) -> Result<()> {
    let agent_id = get_agent_id()?;
    touch_heartbeat(&agent_id)?;

    if is_agent_mode {
        println!(
            "{}",
            serde_json::json!({
                "status": "success",
                "action": "beat",
                "agent_id": agent_id
            })
        );
    } else {
        println!("Heartbeat updated for agent {}", agent_id);
    }

    Ok(())
}

fn cmd_clean_stale(timeout: u64, is_agent_mode: bool) -> Result<()> {
    // Check if it's time to rotate logs
    let now = Utc::now();
    if now.hour() == 0 && now.minute() < 5 {
        rotate_coordination_log()?;
    }

    let cleaned = clean_stale_locks(timeout)?;

    if is_agent_mode {
        println!(
            "{}",
            serde_json::json!({
                "status": "success",
                "action": "clean_stale",
                "cleaned": cleaned
            })
        );
    } else {
        if cleaned.is_empty() {
            println!("No stale locks found");
        } else {
            println!("Cleaned {} stale locks:", cleaned.len());
            for lock in &cleaned {
                println!("  - {}", lock);
            }
        }
    }

    Ok(())
}

fn cmd_quota(add: Option<u64>, is_agent_mode: bool) -> Result<()> {
    let quota_path = PathBuf::from(".rotd/coordination/quota.json");
    let lock_path = PathBuf::from(".rotd/coordination/.lock/quota.lock");

    let result = with_lock_result(&lock_path, || -> Result<QuotaTracker> {
        let mut quota: QuotaTracker = if quota_path.exists() {
            read_json(&quota_path)?
        } else {
            QuotaTracker {
                tokens_used: 0,
                last_reset: Utc::now(),
                requests: 0,
            }
        };

        if let Some(tokens) = add {
            quota.tokens_used += tokens;
            quota.requests += 1;
            write_json(&quota_path, &quota)?;
        }

        Ok(quota)
    })?;

    if is_agent_mode {
        println!("{}", serde_json::to_string(&result)?);
    } else {
        println!("Quota Status:");
        println!("  Tokens used: {}", result.tokens_used);
        println!("  Requests: {}", result.requests);
        println!("  Last reset: {}", result.last_reset);
    }

    Ok(())
}

fn cmd_ls(is_agent_mode: bool, verbose: bool) -> Result<()> {
    let registry_path = PathBuf::from(".rotd/coordination/active_work_registry.json");
    let registry: WorkRegistry = read_json(&registry_path)?;

    if is_agent_mode {
        println!("{}", serde_json::to_string(&registry)?);
    } else {
        println!("Work Registry ({} tasks):", registry.tasks.len());
        println!();

        for task in &registry.tasks {
            let status_str = match task.status {
                WorkStatus::Unclaimed => "[ ]",
                WorkStatus::Claimed => "[~]",
                WorkStatus::Blocked => "[!]",
                WorkStatus::Review => "[?]",
                WorkStatus::Done => "[✓]",
            };

            println!(
                "  {} {} - {} ({})",
                status_str,
                task.id,
                task.title,
                format!("{:?}", task.priority).to_lowercase()
            );

            if verbose {
                if let Some(ref agent) = task.claimed_by {
                    println!("      Claimed by: {}", agent);
                }
                if let Some(ref reason) = task.blocked_reason {
                    println!("      Blocked: {}", reason);
                }
            }
        }
    }

    Ok(())
}
