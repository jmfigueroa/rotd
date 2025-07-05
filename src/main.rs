use anyhow::Result;
use clap::{Parser, Subcommand};

mod agent;
mod audit;
mod cli;
mod common;
mod coord;
mod fs_ops;
mod github;
mod history;
mod human;
mod pss;
mod schema;

use cli::commands::buckle_mode::{BuckleModeArgs, handle_buckle_mode};

#[derive(Parser)]
#[command(name = "rotd")]
#[command(about = "Runtime-Oriented Test Discipline CLI utility")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Agent mode - minimal output, strict validation
    #[arg(long, global = true)]
    agent: bool,

    /// Verbose output (human mode only)
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Show what would be done without making changes
    #[arg(long, global = true)]
    dry_run: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize ROTD structure in current project
    Init {
        /// Force initialization even if .rotd directory exists
        #[arg(short, long)]
        force: bool,
    },

    /// Buckle Mode recovery operations
    BuckleMode(BuckleModeArgs),

    /// Generate PSS score for a task
    Score {
        /// Task ID to score
        task_id: String,
        /// Output format: table, json, or summary
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Display task details
    ShowTask {
        /// Task ID to display
        task_id: String,
    },

    /// List logged lessons in readable format
    ShowLessons {
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
    },

    /// Show audit violations
    ShowAudit {
        /// Number of recent entries to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Agent-oriented commands
    Agent {
        #[command(subcommand)]
        subcommand: AgentCommands,
    },

    /// Check ROTD project health and compliance
    Check {
        /// Fix issues automatically where possible
        #[arg(short, long)]
        fix: bool,

        /// Check if Buckle Mode trigger conditions are met
        #[arg(long)]
        buckle_trigger: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell type: bash, zsh, fish, or powershell
        shell: String,
    },

    /// Update ROTD methodology and templates
    Update {
        /// Check for updates without applying
        #[arg(long)]
        check: bool,
        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,
    },

    /// Upgrade ROTD CLI binary to latest version
    Upgrade {
        /// Check for upgrades without applying
        #[arg(long)]
        check: bool,
        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,
    },

    /// Show version information
    Version {
        /// Show project ROTD version
        #[arg(long)]
        project: bool,
        /// Show latest available version
        #[arg(long)]
        latest: bool,
    },

    /// Validate ROTD artifacts
    Validate {
        /// Validate all schemas
        #[arg(long)]
        all: bool,
        /// Validate specific schema type
        #[arg(long)]
        schema: Option<String>,
        /// Strict validation mode
        #[arg(long)]
        strict: bool,
    },

    /// Multi-agent coordination commands
    Coord {
        #[command(subcommand)]
        subcommand: CoordCommands,
    },

    /// Project primer management commands
    Primer {
        #[command(subcommand)]
        subcommand: PrimerCommands,
    },
}

#[derive(Subcommand)]
enum AgentCommands {
    /// Update task entry from JSON input
    UpdateTask {
        /// Read from file instead of stdin
        #[arg(short, long)]
        file: Option<String>,
        /// Enforce strict schema validation
        #[arg(long)]
        strict: bool,
        /// Trigger PSS scoring after update
        #[arg(long)]
        pss: bool,
        /// Auto-populate updated_at timestamp
        #[arg(long)]
        timestamp: bool,
    },

    /// Append test summary
    AppendSummary {
        /// Test summary file path
        #[arg(short, long)]
        file: String,
    },

    /// Log lesson learned from JSON input
    LogLesson {
        /// Read from file instead of stdin
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Update coverage ratchet
    RatchetCoverage {
        /// New coverage percentage
        coverage: f64,
        /// Task ID associated with coverage update
        #[arg(short, long)]
        task_id: Option<String>,
    },

    /// Show minified command info for LLM agents
    Info,
}

#[derive(Subcommand)]
enum CoordCommands {
    /// Claim the next available task
    Claim {
        /// Filter by capability
        #[arg(long)]
        capability: Option<String>,
        /// Filter by skill level (<=entry, <=intermediate, expert)
        #[arg(long)]
        skill_level: Option<String>,
        /// Claim any task regardless of priority
        #[arg(long)]
        any: bool,
    },

    /// Release a claimed task
    Release {
        /// Task ID to release
        task_id: String,
    },

    /// Approve a task in review status
    Approve {
        /// Task ID to approve
        task_id: String,
    },

    /// Append message to coordination log
    Msg {
        /// Message to log
        message: String,
    },

    /// Update agent heartbeat
    Beat,

    /// Clean stale locks and rotate logs
    CleanStale {
        /// Timeout in seconds (default: 900)
        #[arg(long, default_value = "900")]
        timeout: u64,
    },

    /// Update quota tracker
    Quota {
        /// Add tokens to quota
        #[arg(long)]
        add: Option<u64>,
    },

    /// List current work registry
    Ls,

    /// View task history
    History {
        /// Task ID to view history for
        task_id: String,
        /// Output format: summary (default), json, or stats
        #[arg(long, default_value = "summary")]
        format: String,
    },

    /// Prune old history files
    PruneHistory {
        /// Only show what would be pruned, don't actually prune
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Subcommand)]
enum PrimerCommands {
    /// Initialize primer for current project
    Init {
        /// Force overwrite existing primer
        #[arg(short, long)]
        force: bool,
    },

    /// Show current primer content
    Show {
        /// Show specific primer file
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Validate primer against current project state
    Check,

    /// Parse primer and output structured information for agents
    Parse {
        /// Output format: json or summary
        #[arg(short, long, default_value = "json")]
        format: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Agent mode automatically sets minimal output
    let is_agent_mode = cli.agent || matches!(cli.command, Commands::Agent { .. });

    match cli.command {
        Commands::Init { force } => {
            if is_agent_mode {
                agent::init(force, cli.dry_run)
            } else {
                human::init(force, cli.dry_run, cli.verbose)
            }
        }

        Commands::Score { task_id, format } => {
            if is_agent_mode {
                agent::score(&task_id, &format)
            } else {
                human::score(&task_id, &format, cli.verbose)
            }
        }

        Commands::ShowTask { task_id } => human::show_task(&task_id, cli.verbose),

        Commands::ShowLessons { tag } => human::show_lessons(tag.as_deref(), cli.verbose),

        Commands::ShowAudit { limit } => human::show_audit(limit, cli.verbose),

        Commands::Agent { subcommand } => match subcommand {
            AgentCommands::UpdateTask {
                file,
                strict,
                pss,
                timestamp,
            } => agent::update_task(file.as_deref(), strict, pss, timestamp, cli.dry_run),
            AgentCommands::AppendSummary { file } => agent::append_summary(&file, cli.dry_run),
            AgentCommands::LogLesson { file } => agent::log_lesson(file.as_deref(), cli.dry_run),
            AgentCommands::RatchetCoverage { coverage, task_id } => {
                agent::ratchet_coverage(coverage, task_id.as_deref(), cli.dry_run)
            }
            AgentCommands::Info => agent::info(),
        },

        Commands::Check {
            fix,
            buckle_trigger,
        } => {
            if buckle_trigger {
                if is_agent_mode {
                    agent::check_buckle_trigger()
                } else {
                    human::check_buckle_trigger(cli.verbose)
                }
            } else if is_agent_mode {
                agent::check(fix)
            } else {
                human::check(fix, cli.verbose)
            }
        }

        Commands::Completions { shell } => human::completions(&shell),

        Commands::Update { check, yes } => {
            if is_agent_mode {
                agent::update(check, yes)
            } else {
                human::update(check, yes, cli.verbose)
            }
        }

        Commands::Upgrade { check, yes } => {
            if is_agent_mode {
                agent::upgrade(check, yes)
            } else {
                human::upgrade(check, yes, cli.verbose)
            }
        }

        Commands::Version { project, latest } => {
            if is_agent_mode {
                agent::version(project, latest)
            } else {
                human::version(project, latest, cli.verbose)
            }
        }

        Commands::BuckleMode(buckle_args) => handle_buckle_mode(&buckle_args),

        Commands::Validate {
            all,
            schema,
            strict,
        } => {
            if is_agent_mode {
                agent::validate(all, schema.as_deref(), strict)
            } else {
                human::validate(all, schema.as_deref(), strict, cli.verbose)
            }
        }

        Commands::Coord { subcommand } => {
            coord::handle_command(subcommand, is_agent_mode, cli.verbose)
        }

        Commands::Primer { subcommand } => match subcommand {
            PrimerCommands::Init { force } => {
                if is_agent_mode {
                    agent::primer_init(force)
                } else {
                    human::primer_init(force, cli.verbose)
                }
            }
            PrimerCommands::Show { file } => {
                if is_agent_mode {
                    agent::primer_show(file.as_deref())
                } else {
                    human::primer_show(file.as_deref(), cli.verbose)
                }
            }
            PrimerCommands::Check => {
                if is_agent_mode {
                    agent::primer_check()
                } else {
                    human::primer_check(cli.verbose)
                }
            }
            PrimerCommands::Parse { format } => {
                if is_agent_mode {
                    agent::primer_parse(&format)
                } else {
                    human::primer_parse(&format, cli.verbose)
                }
            }
        }
    }
}
