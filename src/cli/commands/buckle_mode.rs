use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Args)]
pub struct BuckleModeArgs {
    #[clap(subcommand)]
    pub command: BuckleModeCommands,
}

#[derive(Debug, Subcommand)]
pub enum BuckleModeCommands {
    /// Enter Buckle Mode for a specific task
    Enter {
        /// Task ID to fix
        task_id: String,
    },

    /// Generate diagnostic report for current state
    Diagnose,

    /// Fix compilation errors
    #[clap(name = "fix-compilation")]
    FixCompilation,

    /// Fix missing artifacts
    #[clap(name = "fix-artifacts")]
    FixArtifacts,

    /// Check if exit criteria are met
    #[clap(name = "check-exit")]
    CheckExit,

    /// Exit Buckle Mode
    Exit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuckleModeState {
    pub active: bool,
    pub task_id: Option<String>,
    pub entered_at: String,
    pub compilation_fixed: bool,
    pub artifacts_fixed: bool,
    pub exit_criteria_met: bool,
}

/// Handle the buckle-mode command
pub fn handle_buckle_mode(args: &BuckleModeArgs) -> anyhow::Result<()> {
    match &args.command {
        BuckleModeCommands::Enter { task_id } => {
            // Check if in agent mode
            if std::env::args().any(|arg| arg == "--agent") {
                match crate::agent::enter_buckle_mode(task_id) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(anyhow::anyhow!(e.to_string())),
                }
            } else {
                match crate::human::enter_buckle_mode(task_id, false) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(anyhow::anyhow!(e.to_string())),
                }
            }
        }

        BuckleModeCommands::Diagnose => {
            // Check if in agent mode
            if std::env::args().any(|arg| arg == "--agent") {
                match crate::agent::diagnose_buckle_mode() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(anyhow::anyhow!(e.to_string())),
                }
            } else {
                match crate::human::diagnose_buckle_mode(false) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(anyhow::anyhow!(e.to_string())),
                }
            }
        }

        BuckleModeCommands::FixCompilation => {
            // Check if in agent mode
            if std::env::args().any(|arg| arg == "--agent") {
                match crate::agent::fix_compilation() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(anyhow::anyhow!(e.to_string())),
                }
            } else {
                match crate::human::fix_compilation(false) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(anyhow::anyhow!(e.to_string())),
                }
            }
        }

        BuckleModeCommands::FixArtifacts => {
            // Check if in agent mode
            if std::env::args().any(|arg| arg == "--agent") {
                match crate::agent::fix_artifacts() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(anyhow::anyhow!(e.to_string())),
                }
            } else {
                match crate::human::fix_artifacts(false) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(anyhow::anyhow!(e.to_string())),
                }
            }
        }

        BuckleModeCommands::CheckExit => {
            // Check if in agent mode
            if std::env::args().any(|arg| arg == "--agent") {
                match crate::agent::check_exit_criteria() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(anyhow::anyhow!(e.to_string())),
                }
            } else {
                match crate::human::check_exit_criteria(false) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(anyhow::anyhow!(e.to_string())),
                }
            }
        }

        BuckleModeCommands::Exit => {
            // Check if in agent mode
            if std::env::args().any(|arg| arg == "--agent") {
                match crate::agent::exit_buckle_mode() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(anyhow::anyhow!(e.to_string())),
                }
            } else {
                match crate::human::exit_buckle_mode(false) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(anyhow::anyhow!(e.to_string())),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buckle_mode_state_serialization() {
        let state = BuckleModeState {
            active: true,
            task_id: Some("6.2".to_string()),
            entered_at: "2025-07-03T12:00:00Z".to_string(),
            compilation_fixed: false,
            artifacts_fixed: false,
            exit_criteria_met: false,
        };

        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("active"));
        assert!(json.contains("task_id"));

        let deserialized: BuckleModeState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.active, true);
        assert_eq!(deserialized.task_id, Some("6.2".to_string()));
    }
}
