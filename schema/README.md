# ROTD JSON Schema Directory

This directory contains JSON schemas for validating ROTD data structures.

## Core Schemas (v1.0+)

- **task.schema.json** - Task entries in tasks.jsonl
  - Updated in v1.3.0 to include: capability, skill_level, deps, description fields

- **pss_score.schema.json** - Progress Scoring System entries in pss_scores.jsonl

## Multi-Agent Coordination Schemas (v1.3.0)

- **work_registry.schema.json** - Active work registry for task coordination
  - Tracks task status: unclaimed, claimed, blocked, review, done
  - Includes priority, dependencies, and agent assignment

- **dependency_map.schema.json** - Task dependency mapping
  - Simple key-value mapping of task IDs to their prerequisites

- **quota.schema.json** - Token and request usage tracking
  - Monitors resource consumption across multiple agents
  - Optional per-agent breakdown

- **coordination_log.schema.json** - Structured coordination log entries
  - Optional schema for structured logging (plain text also supported)
  - Covers all coordination events

- **lock_file.schema.json** - Lock file metadata
  - Used in both task-level and file-level locks

- **heartbeat.schema.json** - Agent heartbeat file format
  - Tracks agent liveness and current activity

- **agent_config.schema.json** - Agent configuration
  - Defines agent capabilities, skill level, and timing parameters

## Usage

These schemas can be used to validate JSON/JSONL files using any JSON Schema validator:

```bash
# Example using ajv-cli
ajv validate -s schema/task.schema.json -d .rotd/tasks.jsonl --json-lines

# Example using Python jsonschema
python -m jsonschema.cli schema/work_registry.schema.json .rotd/coordination/active_work_registry.json
```

## Version History

- v1.0.0 - Initial schemas (task, pss_score)
- v1.3.0 - Multi-agent coordination schemas and task schema updates