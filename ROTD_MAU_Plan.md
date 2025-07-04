
# ROTD Multi-Agent Update (MAU) Development Plan

This document outlines the development plan for updating the ROTD implementation to support multi-agent coordination, as detailed in `multiagent_coordination_details.md` and `multiagent_coordination_spec.md`.

## 1. Core Objectives

- **Enable Concurrent Agent Operation:** Allow multiple agents to work on the same project simultaneously without conflicts.
- **Prevent Data Corruption:** Ensure the integrity of ROTD artifacts through robust locking mechanisms.
- **Improve Task Management:** Introduce features for task dependencies, prioritization, and review gates.
- **Enhance Observability:** Provide clear logging and tracking of agent activities.

## 2. Development Phases

The implementation will be broken down into the following phases:

### Phase 1: Foundational Locking and Coordination Directory

- **Task 1.1:** Create the `.rotd/coordination` directory structure.
- **Task 1.2:** Implement artifact-level file locking using a `with_lock` helper function in `src/fs_ops.rs`.
- **Task 1.3:** Integrate the locking mechanism into all existing CLI commands that perform write operations.
- **Task 1.4:** Implement the basic `active_work_registry.json` with `unclaimed`, `claimed`, and `done` statuses.
- **Task 1.5:** Implement agent heartbeats and stale-lock recovery.

### Phase 2: Enhanced Task Claiming and Management

- **Task 2.1:** Extend `active_work_registry.json` to include `priority`, `blocked`, and `review` statuses.
- **Task 2.2:** Implement `rotd coord claim` to be priority-aware and respect task dependencies defined in `dependency_map.json`.
- **Task 2.3:** Implement `rotd coord release` to update task status and append a summary to the coordination log.
- **Task 2.4:** Implement the review gate with `rotd coord approve`.

### Phase 3: Advanced Features and Hygiene

- **Task 3.1:** Implement path-scoped file-locking for source code files.
- **Task 3.2:** Implement daily rotation of `coordination.log`.
- **Task 3.3:** Implement the lightweight quota tracker with `rotd coord quota`.

### Phase 4: Testing and Documentation

- **Task 4.1:** Create comprehensive integration tests for all new and modified commands.
- **Task 4.2:** Update all relevant documentation, including `ROTD.md`, `CLI_COMMANDS.md`, and agent prompts.
- **Task 4.3:** Perform a multi-agent soak test to validate the entire system under load.

## 3. Task Breakdown and Tracking

All tasks will be managed using the `.rotd/tasks.jsonl` file, accessible via the `rotd` CLI. Each task will be created with a clear description, priority, and dependencies.

## 4. Initial Tasks

The following tasks will be created immediately to begin the implementation:

- **1.1:** Create the `.rotd/coordination` directory structure.
- **1.2:** Implement the file-locking mechanism in `src/fs_ops.rs`.
- **1.3:** Integrate file-locking into existing CLI commands.

This plan will be updated as the project progresses.

## 5. ADDENDUM 1: Current Status (July 4, 2025)

Based on the current state of the project:
- Tasks 1.1, 1.2, 1.3, and 1.4 have been marked as complete
- However, missing test summaries indicate incomplete ROTD compliance
- The implementation needs to be validated and extended

### Updated Task Structure

The original tasks need to be reorganized to include:
- Capability-aware routing (from multiagent_coordination_spec.md Addendum-B)
- Extended schema fields for task seeding
- Agent capability declarations

## 6. ADDENDUM 2: Extended Feature Set

Based on the specifications, the following features need to be added:

### Phase 1.5: Capability-Aware Task Routing
- **Task 1.5.1:** Extend task schema with `capability` and `skill_level` fields
- **Task 1.5.2:** Implement capability filtering in `rotd coord claim`
- **Task 1.5.3:** Add environment variable support for agent capabilities
- **Task 1.5.4:** Create task seeding command `rotd agent seed-tasks`

### Phase 2.5: Enhanced Coordination Features
- **Task 2.5.1:** Implement blocked status with reason tracking
- **Task 2.5.2:** Add automated release summaries to coordination log
- **Task 2.5.3:** Implement dependency validation in claim logic

## 7. ADDENDUM 3: Implementation Notes

### Key Changes from Original Spec:
1. **File Locking**: Must use `fs2` crate for cross-platform compatibility
2. **Lock Directory Structure**: 
   - `.rotd/.lock/` for standard artifacts
   - `.rotd/coordination/.lock/` for coordination-specific locks
3. **Agent ID Management**: Agents need persistent UUIDs stored in environment or config
4. **Heartbeat Mechanism**: 60-second intervals with 15-minute timeout threshold

### Migration Strategy:
1. Create migration command in CLI to update existing projects
2. Preserve backward compatibility for single-agent workflows
3. Add validation for new schema fields

## 8. Next Immediate Tasks

Based on the current state, the following tasks should be created and worked on:

- **Task 1.5:** ✅ Implement agent heartbeats and stale-lock recovery (COMPLETE)
- **Task 1.6:** Add test coverage for completed locking mechanisms
- **Task 2.1:** Extend registry schema with new status fields
- **Task 2.2:** Implement priority-aware claiming logic
- **Task 2.3:** Add dependency validation to claim logic

## 9. ADDENDUM 4: Implementation Progress (July 4, 2025)

### Completed in this session:

1. **Multi-Agent Coordination Module (coord.rs)**:
   - Implemented complete coordination command structure
   - Added all planned subcommands: claim, release, approve, msg, beat, clean-stale, quota, ls
   - Created data structures for WorkRegistry, TaskPriority, WorkStatus
   - Implemented heartbeat mechanism with configurable timeouts
   - Added priority-aware task claiming (urgent > high > medium > low)
   - Implemented dependency checking in claim logic
   - Added coordination logging with automatic message formatting

2. **File Locking Enhancement**:
   - Created `with_lock_result` generic function to support returning values from locked operations
   - Integrated locking into coordination operations
   - Added proper lock directory creation

3. **Integration with Main CLI**:
   - Added `coord` command to main CLI structure
   - Properly integrated with agent/human mode detection
   - All commands support both JSON output (agent mode) and human-readable output

### Key Implementation Details:

- **Agent ID Management**: Uses UUID v4, can be set via `ROTD_AGENT_ID` environment variable
- **Heartbeat Files**: Stored in `.rotd/coordination/heartbeat/<agent_id>.beat`
- **Lock Files**: Task locks in `.rotd/coordination/agent_locks/<task_id>.<agent_id>.lock`
- **Registry Locks**: Uses `.rotd/coordination/.lock/registry.lock` for atomic operations
- **Coordination Log**: Appends to `.rotd/coordination/coordination.log` with ISO timestamps

### Remaining Work:

1. **Testing**: Need to create actual integration tests for concurrent operations
2. **Documentation**: Update CLI documentation with new coord commands
3. **Schema Updates**: Formalize JSON schemas for new structures
4. **Review Gates**: Implement the review workflow (currently stubbed)
5. **Quota Tracking**: Enhance quota tracking with reset logic
6. **Log Rotation**: Implement automatic daily log rotation
7. **Skill Level Filtering**: Complete the skill level comparison logic
