# Multi-Agent Coordination Examples (v1.3.0)

This directory contains example files for ROTD's multi-agent coordination features introduced in v1.3.0.

## New Coordination Files

### active_work_registry.json
Central registry of all tasks being worked on by agents. Each task includes:
- `capability`: Type of work (frontend_ts, backend_rust, tests_only, docs, refactor)
- `skill_level`: Required skill level (entry, intermediate, expert)
- `status`: Current status (unclaimed, claimed, blocked, review, done)
- `claimed_by`: Agent ID that claimed the task
- `reviewer_id`: Agent ID reviewing completed work

### dependency_map.json
Maps task IDs to their dependencies. Format:
```json
{
  "task_id": ["dependency1", "dependency2"]
}
```

### coordination.log
Append-only log of all coordination events:
- CLAIM: Agent claims a task
- RELEASE: Agent releases a task
- COMPLETE: Agent completes a task
- REQUEST_REVIEW: Task ready for review
- APPROVE: Reviewer approves task
- BLOCKED: Task blocked with reason
- HEARTBEAT: Agent is still active
- STALE_CHECK: Monitor checks for stale agents
- DEPENDENCY_CHECK: Verify dependencies satisfied
- QUOTA_CHECK: API quota status

### quota.json
Tracks API usage to prevent quota exhaustion:
- `tokens_used`: Total tokens used since last reset
- `last_reset`: When the quota period started
- `requests`: Number of API requests made

### Lock Files
- `registry.lock`: Prevents concurrent registry modifications
- `*.beat`: Heartbeat files (empty, existence indicates agent is alive)

## Task Field Updates

Regular tasks in `tasks.jsonl` remain unchanged. The capability and skill_level fields are only used in the coordination registry to help agents find appropriate work.

## Usage Examples

```bash
# Agent claims a frontend task
rotd coord claim --capability frontend_ts --skill_level intermediate

# Agent releases a blocked task
rotd coord release task-id

# Agent marks task complete and requests review
rotd coord complete task-id

# Senior agent approves reviewed task
rotd coord approve task-id

# Check coordination status
rotd coord ls --verbose
```

## Coordination Workflow

1. Agents check `active_work_registry.json` for unclaimed tasks
2. Agent claims task matching their capability/skill
3. Agent sends heartbeats every 15 minutes
4. On completion, agent requests review
5. Another agent reviews and approves
6. Task marked as done in registry

All operations use file locking to prevent conflicts between concurrent agents.