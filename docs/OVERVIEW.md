# ROTD Overview

> Essential guide to Runtime-Oriented Test Discipline

## What is ROTD?

ROTD (Runtime-Oriented Test Discipline) is a test-anchored, artifact-driven development methodology optimized for LLM-led development. It ensures that runtime validation is the single source of truth while maintaining comprehensive project tracking and learning.

## Core Principles

1. **Test-Driven**: Every feature needs tests that prove it works
2. **Runtime Truth**: 100% tests must pass before marking tasks complete
3. **Clean Code**: No TODOs or stubs on main branch
4. **Systematic Progress**: Track everything in structured artifacts
5. **Continuous Learning**: Log failures for future reference

## Artifact File Locations

All ROTD artifacts are stored in the `.rotd/` directory:

```
.rotd/
├── tasks.jsonl              # Task tracking (append-only)
├── test_summaries/          # Test results per task
│   └── <task_id>.json
├── lessons_learned.jsonl    # Failure patterns & solutions
├── pss_scores.jsonl         # Progress scoring results
├── session_state.json       # Current task & context
├── coverage_history.json    # Test coverage tracking
├── audit.log                # Violations & overrides
└── coordination/            # Multi-agent support (v1.3+)
    ├── active_work_registry.json
    ├── dependency_map.json
    ├── coordination.log
    ├── quota.json
    ├── agent_locks/
    ├── file_locks/
    ├── heartbeat/
    └── .lock/
```

## Essential CLI Commands

### Project Setup
```bash
rotd init                    # Initialize ROTD project
rotd check                   # Verify project health
rotd check --fix             # Auto-fix issues where possible
```

### Task Management
```bash
rotd show-task <task_id>     # View task details
rotd score <task_id>         # Generate PSS score
echo '{"id":"X.Y","status":"complete"}' | rotd agent update-task --timestamp
```

### Information & Learning
```bash
rotd show-lessons            # View lessons learned
rotd show-audit --limit=10   # Recent audit entries
rotd agent log-lesson        # Record new lesson
```

### Multi-Agent Coordination (v1.3+)
```bash
rotd coord claim             # Claim next available task
rotd coord release <task_id> # Release completed task
rotd coord beat              # Update heartbeat
rotd coord ls                # View work registry
```

## Task Lifecycle

1. **Scaffolded**: Task created but not started
2. **In Progress**: Actively being worked on
3. **Complete**: All tests pass, artifacts logged
4. **Blocked**: Waiting on dependencies or review (v1.3+)
5. **Review**: Awaiting approval (v1.3+)

## Progress Scoring System (PSS)

Tasks are scored on a 10-point scale:

- **1-3**: Execution Sanity (engagement, compilation, implementation)
- **4-6**: Testing Discipline (tests written, passing, quality)
- **7-8**: Cleanup Discipline (no stubs, documentation)
- **9-10**: Historical Continuity (artifacts maintained, lessons logged)

**Passing threshold**: Score ≥ 6

## Quick Start Workflow

```bash
# 1. Initialize project
rotd init

# 2. Check health
rotd check

# 3. Work on task
rotd show-task 1.1
# ... implement feature and tests ...

# 4. Complete task
echo '{"id":"1.1","status":"complete"}' | rotd agent update-task --timestamp
rotd agent append-summary --file test_summaries/1.1.json

# 5. Score progress
rotd score 1.1

# 6. Learn from errors
echo '{"id":"err-001","diagnosis":"...","remediation":"..."}' | rotd agent log-lesson
```

## Multi-Agent Development (v1.3+)

When multiple agents work together:

```bash
# Agent 1
export ROTD_AGENT_ID=agent-1
rotd coord beat
rotd coord claim --capability backend_rust
# ... work on task ...
rotd coord release <task_id>

# Agent 2
export ROTD_AGENT_ID=agent-2
rotd coord claim --capability tests_only
```

## Key Rules

- **Never** mark a task complete without passing tests
- **Always** use CLI commands (never edit .rotd files manually)
- **Document** failures as lessons for future sessions
- **Maintain** clean code (no stubs or TODOs on main)
- **Track** all work through task IDs

## Getting Help

```bash
rotd --help              # General help
rotd <command> --help    # Command-specific help
rotd agent info          # Agent command reference
```

For detailed methodology: See [ROTD.md](./ROTD.md)
For CLI reference: See [CLI_COMMANDS.md](./CLI_COMMANDS.md)