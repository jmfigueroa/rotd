# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Task History Tracking**: Comprehensive audit trail for task status changes
  - `.rotd/task_history/` directory with per-task JSONL files
  - Automatic history recording on every task update
  - Agent ID and timestamp tracking for all changes
  - Previous status retention for audit trail
  - Optional comments and PSS score deltas
  - `rotd coord history <task_id>` command with multiple output formats (summary, json, stats)
  - `rotd coord prune-history` command for history management
  - Configurable history size limits and compression settings
- **ROTD Configuration File**: `.rotd/config.jsonc` with JSONC support
  - `history_max_size_mib`: Maximum uncompressed size per task history file
  - `history_compress_closed`: Auto-compress completed task histories
  - `history_total_cap_mib`: Hard cap on total history directory size
- Advanced quota management features (planned)
- Task graph visualization (planned)
- Distributed coordination support (planned)

## [1.3.0] - 2025-07-04

### Added
- **Multi-Agent Coordination System**: Complete support for parallel development with multiple agents
  - `rotd coord` command suite: claim, release, approve, msg, beat, clean-stale, quota, ls
  - Agent heartbeat mechanism with 60-second intervals and 15-minute timeout
  - Task claiming with capability and skill level filtering
  - Priority-aware task assignment (urgent > high > medium > low)
  - Dependency validation during task claiming
  - Work registry with full task lifecycle tracking
  - Coordination logging with timestamps and agent IDs
- **Artifact-Level File Locking**: Prevents concurrent write conflicts
  - Generic `with_lock_result` function for locked operations
  - Lock metadata with holder and timestamp information
  - Automatic stale lock recovery
- **Enhanced Task Schema**: New fields for multi-agent routing
  - `capability` field: frontend_ts, backend_rust, tests_only, docs, refactor
  - `skill_level` field: entry, intermediate, expert
- **Coordination Directory Structure**: `.rotd/coordination/`
  - `active_work_registry.json`: Task status and ownership
  - `dependency_map.json`: Task prerequisite tracking
  - `coordination.log`: Inter-agent communication
  - `quota.json`: Usage tracking
  - `agent_locks/`: Task-level locks
  - `file_locks/`: Path-scoped locks (prepared)
  - `heartbeat/`: Agent liveness tracking
  - `.lock/`: Artifact locks

### Changed
- Updated prompts.md with multi-agent coordination prompts
- Enhanced CLI_COMMANDS.md with coord subcommands
- Added environment variable support for agent configuration
- Improved error codes to include lock timeout (4) and stale lock recovery (5)

### Fixed
- Rust borrow checker issues in coordination operations
- Lock file creation race conditions
- Task status synchronization between registry and tasks.jsonl

## [1.2.1] - 2025-07-03

### Added
- Task prioritization system with 5-level priority field
- Optional priority_score for fine-grained ranking
- Periodic review process for project health
- Review schedule tracking in `.rotd/review_schedule.json`
- Enhanced schema validation for new fields

### Changed
- Updated task schema to include priority information
- Enhanced update protocol for smoother migrations
- Improved CLI update command with manifest generation

## [1.2.0] - 2025-07-02

### Added
- Complete ROTD Update Protocol documentation
- Version tracking and comparison commands
- Update manifest generation for migrations
- Post-update user guidance with copy-pastable prompts
- Update history tracking in `.rotd/update_history.jsonl`
- Backup functionality before updates
- Enhanced validation commands (`rotd validate`)

### Changed
- Improved `rotd update` command with --check flag
- Better error handling for network failures
- Enhanced user prompts for update application

## [1.1.0] - 2025-07-01

### Added
- Buckle Mode recovery protocol for compilation and artifact integrity failures
- Audit rule for automatic Buckle Mode triggering (audit.buckle.trigger.001)
- CLI commands for Buckle Mode diagnostics and recovery
- Enhanced task tracking protection with session boundary enforcement
- Complete ROTD CLI utility with agent and human modes
- Agent-aware prompts for LLM workflows
- JSON schema validation for all ROTD artifacts
- Comprehensive test suite with integration tests
- CI/CD pipeline with automated releases
- Cross-platform binary builds (Linux, macOS, Windows)
- Shell completion support for bash, zsh, fish, and PowerShell
- Audit logging for all ROTD operations
- PSS (Progress Scoring System) integration
- Coverage ratchet mechanism
- Dry-run mode for safe operations
- Comprehensive documentation and examples

### Changed
- Restructured project for dual CLI/manual operation
- Enhanced prompts with CLI-specific instructions
- Improved error handling and validation
- Updated documentation structure

### Fixed
- Schema validation edge cases
- File operation safety and concurrency
- Cross-platform compatibility issues

## [1.0.0] - 2025-06-30

### Added
- Initial stable release of ROTD methodology
- Core artifact structure (tasks.jsonl, test_summaries/, etc.)
- Progress Scoring System (PSS) implementation
- Lessons learned tracking
- Session state management
- Coverage history tracking
- Audit logging capabilities

### Changed
- Stabilized artifact formats
- Finalized scoring criteria
- Standardized workflow processes

## [0.1.0] - 2025-06-15

### Added
- Initial ROTD methodology documentation
- Basic Python PSS scoring script
- JSON schemas for validation
- Example ROTD artifacts
- Manual workflow prompts

[Unreleased]: https://github.com/jmfigueroa/rotd/compare/v1.3.0...HEAD
[1.3.0]: https://github.com/jmfigueroa/rotd/compare/v1.2.1...v1.3.0
[1.2.1]: https://github.com/jmfigueroa/rotd/compare/v1.2.0...v1.2.1
[1.2.0]: https://github.com/jmfigueroa/rotd/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/jmfigueroa/rotd/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/jmfigueroa/rotd/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/jmfigueroa/rotd/releases/tag/v0.1.0