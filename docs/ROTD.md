# ROTD — Runtime-Oriented Test Discipline  
*Test-driven development optimized for LLM workflows, where runtime validation is the single source of truth.*

## Core Philosophy

Traditional TDD: "Write the failing test first."  
ROTD: "Explore and implement, but nothing is done until tests prove it works."

The runtime—**what actually executes and passes**—is the single source of truth.

## Project Structure

Create a `.rotd/` directory (note the leading dot):

```
.rotd/                       # ROTD-specific directory (hidden)
├── tasks.jsonl              # Append-only task log
├── test_summaries/          # Proof of completion: <task_id>.json
├── task_history/            # Per-task change history: <task_id>.jsonl
├── lessons_learned.jsonl    # Reusable failure/fix patterns
├── audit.log                # Rule violations (JSONL)
├── stub_report.json         # CI-generated stub tracking
├── session_state.json       # Delta prompting & efficiency tracking
├── coverage_history.json    # Adaptive coverage progression
├── pss_scores.jsonl         # Progress Scoring System results
└── config.jsonc             # ROTD configuration with history management settings
```

## Core Rules

| Rule | Description | Required Artifact | Enforcement |
|------|-------------|-------------------|-------------|
| **R1** | Every feature needs tests | `tasks.jsonl` with test paths | Block merge |
| **R2** | Stubs require failing tests + `#[rotd_stub]` | `stub_report.json` | Block if stubs on main |
| **R3** | "Complete" = 100% tests pass | `test_summaries/<task>.json` | Block merge |
| **R4** | No TODOs on main | `grep -R "TODO("` scan | Block merge |
| **R5** | Breaking changes need rationale | `rationales/<task>.md` | Manual review |
| **R6** | Log all violations | `audit.log` | CI artifact |
| **R7** | Cross-phase integration required | Integration tasks in `tasks.jsonl` | Block merge |

## Task Lifecycle

1. **Define** → Add task to `tasks.jsonl`
2. **Test** → Write failing tests (or stub + test)
3. **Implement** → Code until tests pass
4. **Verify** → Generate `test_summaries/<task>.json`
5. **Commit** → Include `Task-ID:` in message
6. **Learn** → Log failures to `lessons_learned.jsonl`

## Key Artifacts

### `tasks.jsonl`
Append-only log, one JSON object per line:
```json
{"id":"4.2","phase":"4","title":"PreambleEditor modal","status":"in_progress",
 "tests":["tests/P4/preamble_modal.test.tsx"],"created":"2025-07-01T05:00Z",
 "priority":"high"}
{"id":"cross-1","phase":"X","title":"Modal shortcuts integration","status":"pending",
 "depends_on":["4","6"],"tests":["__tests__/cross-phase/modal-shortcuts.test.tsx"],
 "priority":"medium","priority_score":65.0}
```

**Task Priority**: Each task can include:
- `priority`: One of `urgent`, `high`, `medium`, `low`, `deferred`
- `priority_score`: Optional numeric score (0-100) for finer-grained ranking

**Cross-phase integration tasks**: Use `phase:"X"` and include `depends_on` array listing prerequisite phases. Must include comprehensive integration test paths covering multi-phase interactions.

### Test Summary
```json
{
  "task_id": "4.2",
  "total": 36,
  "passed": 36,
  "failed": 0,
  "coverage": 0.91,
  "runner": "jest",
  "timestamp": "2025-07-01T05:45:12Z"
}
```

### Audit Log
```json
{"ts":"2025-07-01T05:05Z","task":"4.1","rule":"R3_FALSE_PASS","severity":"error",
 "msg":"Task claimed complete with 24/32 tests passing"}
```

### Lessons Learned
```json
{"id":"TRACE_NO_FOOTER","first":"2025-06-30","root":"Missing Task-ID footer",
 "fix":"Git commit.template + pre-commit hook","verify":"passes rotd_check"}
```

### Task History
Each task has its own history file in `.rotd/task_history/<task_id>.jsonl` that tracks:
- Status changes with timestamps and agent IDs
- Comments explaining changes
- PSS score deltas
- Previous status for audit trail

Example entry:
```json
{"task_id":"6.2","agent_id":"claude-20250105","timestamp":"2025-01-05T10:00:00Z",
 "status":"complete","prev_status":"in_progress","comment":"All tests passing",
 "pss_delta":2.0,"schema":"task_history.v1"}
```

### Config File
`.rotd/config.jsonc` supports configuration with comments:
```jsonc
{
  // Max uncompressed size per task history before rotation (MiB)
  "history_max_size_mib": 1,
  // Compress closed tasks? ("closed" means status == "complete")
  "history_compress_closed": true,
  // Hard cap on total history directory size (MiB)
  "history_total_cap_mib": 100
}
```

## "Now & Later" Stubs

When implementation details are unclear:
1. Create stub with `#[rotd_stub]` annotation
2. Write failing test that defines expected behavior
3. Implement later when requirements clarify

```typescript
// #[rotd_stub(task_id="4.2", reason="API pending")]
function fetchUserPrefs(): UserPrefs {
  throw new Error("STUB: see __tests__/userPrefs.test.ts")
}
```

## LLM Workflow

### Core Protocol
1. **Always report test results** (no silent failures)
2. **Check lessons_learned.jsonl** before debugging (use hash lookup for speed)
3. **Update task status** in real-time
4. **Log violations** to audit.log immediately
5. **Before starting integration work**: Check that prerequisite phases are complete and verify no existing conflicts exist
6. **Cross-phase testing**: Run all affected phase tests when making integration changes to prevent regressions
7. **NEVER use in-memory task tracking**: Always update `.rotd/tasks.jsonl` after each session run
8. **Enter Buckle Mode** when compilation or artifact integrity breaks down

### Task Prioritization
When selecting next task, evaluate:
1. **Blocking status**: Tasks blocking others → `urgent`
2. **Critical path**: Tasks essential for current milestone → `high`
3. **Stalled progress**: In-progress tasks lingering → raise priority
4. **Test failures**: Known regressions → `urgent`
5. **Maintenance work**: Refactoring/docs → `medium` or `low`

Avoid `urgent` overuse. Use `deferred` for intentionally postponed work.

### Efficiency Optimizations

**Delta-Only Prompting**: Store last diff and failing test in `session_state.json`. Include only changed lines (≤50) in prompts to reduce token usage by ~70%.

**Auto-Test Scaffolding**: Use `bin/scaffold_tests.sh` to generate test skeletons from function signatures, triggered by pre-commit hooks.

**Flaky Test Detection**: Wrapper runs failing tests 3x; marks `intermittent` if inconsistent to prevent debugging ghost failures.

**Smart Coverage**: `coverage_guard.py` auto-adjusts floor by +1% when tasks merge with >3% headroom, allowing MVP tests early.

**Cached Lesson Lookup**: Hash stack traces for instant failure pattern matching in `lessons_learned.jsonl`.

**Performance Telemetry**: Track `tokens_used` and `elapsed_ms` in session state for automated prompt optimization.

## CI Integration

```bash
set -e
# Core testing
cargo test --all --locked
npm test -- --runInBand
# ROTD validation
python scripts/gen_stub_report.py && test ! -s .proj_mgmt/stub_report.json
python scripts/verify_summaries.py
grep -R "TODO(" src/ && exit 1 || true
# Efficiency tools
bin/scaffold_tests.sh --check-only
python scripts/coverage_guard.py --validate
```

## Efficiency Optimizations

### Session State Tracking
Store in `.proj_mgmt/session_state.json`:
```json
{
  "last_diff": "src/component.ts:42-48",
  "failing_test": "should handle focus events",
  "tokens_used": 1250,
  "elapsed_ms": 890,
  "coverage_floor": 85
}
```

### Automated Tooling
- **Test Scaffolding**: `bin/scaffold_tests.sh` auto-generates test skeletons
- **Flaky Detection**: Test runner identifies intermittent failures automatically  
- **Coverage Guard**: `coverage_guard.py` dynamically adjusts coverage requirements
- **Hash Lookup**: Generate `sha256` of `stack_trace + test_name` for instant lesson matching

### Context Management
- Include only delta patches (≤50 lines) in prompts
- Reference docs by path instead of inlining
- Use structured JSON for all state tracking
- Track performance metrics to optimize prompt length

## Human Review Points

- Deduplicate lessons_learned.jsonl periodically
- Verify test summaries match reality
- Adjust coverage thresholds as needed

## Periodic ROTD Review

### Frequency
- **Weekly**: Minimum baseline
- **Milestone-based**: At major feature completion
- **On-demand**: When drift suspected

### Review Checklist
1. **Artifact Health**: Are all `.rotd/` files present and updated?
2. **ROTD Compliance**: Tasks marked complete only with passing tests?
3. **Project Alignment**: Do tasks match roadmap priorities?
4. **Drift Detection**: Any incomplete tasks lingering? Audit violations unaddressed?
5. **Corrective Actions**: Reprioritize tasks, extract lessons, address score weaknesses

### Review Output
Generate markdown report with:
- Artifact health status
- Compliance violations found
- Alignment with project goals
- Drift signals detected
- Recommended corrections

## Recovery Protocols

### Buckle Mode

When a project breaks down due to compilation failures or artifact integrity issues, ROTD provides a structured recovery protocol called **Buckle Mode**.

See [ROTD Buckle Mode](./recovery/BUCKLE_MODE.md) for the complete recovery protocol and [Buckle Mode Trigger Rule](./audit_rules/BUCKLE_MODE_TRIGGER.md) for the audit rule that enforces it.

### Key Features

- **Automatic Triggering**: Enters recovery mode when compilation fails or artifacts are missing
- **Structured Recovery**: Clear step-by-step process to restore project health
- **Artifact Protection**: Prevents loss of project state and context
- **Task Integrity**: Maintains `.rotd/tasks.jsonl` as the source of truth

## Summary

ROTD provides structure without rigidity. Tests are mandatory, but the path to get there is flexible. Every claim is backed by executable proof.

## Adoption Path for Optimizations

Implement efficiency features incrementally as cross-efficiency tasks:

1. **`cross-eff-1`**: Session state tracking in `session_state.json`
2. **`cross-eff-2`**: Auto-scaffold test skeleton generation
3. **`cross-eff-3`**: Flaky test detection wrapper
4. **`cross-eff-4`**: Adaptive coverage guard implementation
5. **`cross-eff-5`**: Hash-based lessons learned lookup
6. **`cross-eff-6`**: Performance telemetry integration

Each optimization compounds: fewer tokens → lower cost → more iterations per quota window.

## Lessons Learned System

### Why It Matters

LLMs often repeat mistakes if not given persistent memory between sessions. A shared, structured log of failures and their remedies enables:

- Faster recovery after crashes
- Avoiding re-debugging the same class of issues
- Encoding tribal knowledge into machine-usable artifacts

### Requirements

- **Location**: `.rotd/lessons_learned.jsonl`
- **Format**: JSONL for efficient streaming
- **Schema**:

```json
{
  "id": "ll-2024-07-01-a",
  "hash": "sha256:8f4a9b2c...",
  "trigger": ["TypeError", "undefined is not a function", "ref is null"],
  "context": {
    "task_id": "4.2",
    "component": "PreambleEditor",
    "language": "TypeScript"
  },
  "diagnosis": "MDEditor mock does not support focus ref forwarding in test suite",
  "remediation": "Use custom test mock with forwardRef and stub `.focus()` method.",
  "tags": ["focus", "testing", "mocks", "typescript"]
}
```

### LLM Usage

1. On test failure, **hash the stack trace + test name** to generate lookup key (`LL_HASH`)
2. **Search `.rotd/lessons_learned.jsonl`** for exact `hash` match first, then fallback to triggers/tags
3. If relevant, apply the prior fix and **annotate it** in the task log (`tasks.jsonl`) with the reused `id`
4. If **no match**, after resolution, **append a new entry** with hash for future exact matching

### Benefits

- Prevents repeating past mistakes
- Enables quick pattern matching: "Fixed this before in lesson X"
- Builds institutional knowledge across sessions

### Tooling

- Use `jq` for command-line queries
- Index triggers and tags for fast lookup

### Maintenance

- Max entry size: ~1KB (to reduce prompt pollution)
- Periodically dedupe semantically similar lessons
- Allow `deprecated: true` flag for outdated entries

## Test Integrity Enforcement

### Why It Matters

False positives in test reports erode trust in automation and allow defects to pass undetected. LLMs (and humans) are prone to overconfidence — especially when partial functionality appears to work. Strict validation of test completion is essential for maintaining development integrity in LLM-driven workflows.

### Requirements

All artifacts in `.proj_mgmt/`:
  - `.proj_mgmt/tasks.jsonl` — Canonical task list
  - `.proj_mgmt/test_summaries/` — One file per completed task
    - e.g., `.proj_mgmt/test_summaries/4.2.json`
  - `.proj_mgmt/audit.log` — ROTD violation log
  - `.proj_mgmt/lessons_learned.jsonl` — Postmortem index

### Test Summary Format

Each test summary must be written by the LLM and stored in `.proj_mgmt/test_summaries/{task_id}.json`.

Example (`.proj_mgmt/test_summaries/4.2.json`):

```json
{
  "task_id": "4.2",
  "status": "complete",
  "total_tests": 36,
  "passed": 36,
  "failed": 0,
  "warnings": ["act() usage unwrapped", "low mock fidelity on MDEditor"],
  "coverage": "89%",
  "verified_by": "Claude Code",
  "timestamp": "2025-07-01T04:22:00Z",
  "notes": "All required assertions implemented. Confirmed ARIA compliance. Manual keyboard navigation test pending."
}
```

### Enforcement

Before any task may be marked **`complete`**, the following must be true:

- A `.proj_mgmt/test_summaries/{task_id}.json` file exists.
- `status` must be `"complete"`.
- `passed === total_tests` and `failed === 0`
- LLM must self-report the summary to user and log metadata to `audit.log` if anything fails.

### Violation Example

```text
[2025-07-01T04:19:12Z] [ROTD-K2] Task 4.1 falsely reported as complete: 24/32 tests failed. Summary missing.
```

### Prevention Tactics

- Force all test suites to exit non-zero on fail (`CI=true npm test`, `cargo test -- --fail-fast`)
- Disable "soft" pass heuristics (e.g., "most tests passed = OK")
- Prohibit advancing to next task unless `.rotd/test_summaries/{task_id}.json` is written and parsed

### LLM Directives

- NEVER declare a task complete unless test summary validates 100% pass rate.
- IF any test fails, do NOT proceed to next task — instead:
  - Log failure
  - Re-enter RED/YELLOW TDD cycle
  - Record accurate test summary
- AFTER fix, regenerate summary and append to `.rotd/test_summaries/`.

Test results are first-class artifacts. No task is complete without verified, passing tests.

# 🔢 ROTD Progress Scoring System

> A points-based rubric for measuring meaningful, LLM-aware task progress — grounded in ROTD principles, not shallow metrics.

Each task is scored out of **10 points**, grouped into four thematic domains for visual clarity and diagnostic use. These metrics reinforce ROTD's hybrid approach: letting LLMs operate flexibly, while still maintaining rigorous, auditable development discipline.

## 📍 Location for Scores

Progress scores are stored in:
```
.rotd/pss_scores.jsonl
```

This file is a **JSON Lines** file where each line represents the score for a specific task or session.

## 🧱 JSON Structure (per task)

Each line must contain:

```json
{
  "task_id": "6.2",
  "score": 7,
  "timestamp": "2025-07-02T05:42:10Z",
  "criteria": {
    "llm_engaged": {
      "score": 1,
      "rationale": "Claude iterated three times on shortcut context handlers."
    },
    "compiles": {
      "score": 1,
      "rationale": "Frontend compiled without TypeScript errors."
    },
    "core_impl": {
      "score": 1,
      "rationale": "Shortcut context and modal dispatch implemented per spec."
    },
    "tests_written": {
      "score": 1,
      "rationale": "83 tests written across 5 files; coverage guard triggered."
    },
    "tests_pass": {
      "score": 0,
      "rationale": "Test env failed to mount HelpModal due to router conflicts."
    },
    "doc_maintained": {
      "score": 1,
      "rationale": "All public methods documented; `typedoc` run passed."
    },
    "stub_free": {
      "score": 1,
      "rationale": "All #[rotd_stub] annotations removed by end of session."
    },
    "history_maintained": {
      "score": 1,
      "rationale": "`tasks.jsonl` updated; `session_state.json` saved."
    },
    "qts_floor": {
      "score": 0,
      "rationale": "Coverage fell below historical floor of 73%."
    },
    "qts_ratchet": {
      "score": 0,
      "rationale": "No headroom to trigger ratchet increase."
    }
  }
}
```

## 🧪 Evaluation Timing

- Run this scoring procedure **at the end of each LLM-assisted coding session** (or at the end of a phase).
- Use current test results, session logs, and project file state to justify each score.

## 🔁 Cross-Referencing

Use data from:
- `.rotd/test_summaries/{task_id}.json`
- `.rotd/tasks.jsonl`
- `.rotd/coverage_history.json`
- `.rotd/session_state.json`
- `.rotd/lessons_learned.jsonl` *(optional)*

---

## 🎛 Execution Sanity (1–3)

### 1. LLM Engagement
✅ *"Has the LLM begun work on this task?"*

- Score if the task has either been explicitly started in a session or referenced during resumed output.
- Also earned if the task was auto-generated during decomposition and picked up without prompting.

**Why it matters:** ROTD emphasizes *LLM-led momentum*. No engagement = no useful action.

---

### 2. Compiles
✅ *"Does the codebase build cleanly after this task's output?"*

- Applies to Rust (`cargo check`), TS (`tsc` or similar), or full-stack CI build pipeline.
- Minor warnings acceptable if documented in the test summary.

**Why it matters:** Prevents broken intermediate states and maintains LLM recoverability between sessions.

---

### 3. Core Implementation Complete
✅ *"Is the business logic or primary functionality present?"*

- Look for class/function bodies, not just signatures.
- Should capture the user-meaningful behavior, even if edge cases aren't polished.

**Why it matters:** Ensures tasks aren't "test-passing" shells or placeholder artifacts.

---

## 🧪 Testing Discipline (4–6)

### 4. Tests Written
✅ *"Are there tests present that assert expected behavior?"*

- Not required to pass yet — just exist with meaningful assertions.
- Snapshot tests, unit tests, or integration tests all qualify.

**Why it matters:** Encourages the RED step of TDD or "Now" in 🍬 N&L pattern.

---

### 5. Tests Pass (Threshold Met)
✅ *"Do enough tests pass to consider the feature viable?"*

- Threshold may vary — e.g., ≥70% of new tests passing.
- Failing edge cases are acceptable if documented in `.proj_mgmt/test_summaries/{task_id}.json`.

**Why it matters:** Passing tests are the **primary functional proof** in LLM workflows.

---

### 6. Quality Trajectory Score (QTS)
✅ 1 point for each:

- **6A: Coverage Floor Met**  
  `coverage_guard.py` confirms historical coverage % floor is maintained.

- **6B: Ratchet Triggered**  
  A task merges with headroom > 3%, bumping the project's coverage floor.

**Why it matters:** Encourages long-term test rigor **without strict gates** that hinder LLM output velocity.

---

## 🧹 Cleanup Discipline (7–8)

### 7. Stub-Free
✅ *"Are there any TODOs, stubs, or partial implementations still present?"*

- Must remove `todo!()`, `unimplemented!()`, empty bodies, and placeholder returns.
- In TS: `throw new Error("TODO")` or `// TODO` comments must be resolved.

**Why it matters:** Stub rot is a known LLM risk. Enforces production readiness.

---

### 8. Documentation Maintenance
✅ *"Are docs, code comments, and formatters in a good state?"*

- Must pass `rustdoc`/`typedoc` generation (if applicable).
- Key functions/modules/classes should have summaries.
- Lint and format passes required (`cargo fmt`, `eslint`, `prettier`).

**Why it matters:** Ensures LLM and human maintainability across phases.

---

## 🗂 Historical Continuity (9–10)

### 9. Project History Maintained
✅ *"Are all project artifacts updated and traceable?"*

- `tasks.jsonl` reflects the task with correct phase, status, origin.
- `test_summaries/{task_id}.json` exists and is informative.
- `lessons_learned.jsonl` updated if new failure pattern emerged.
- `session_state.json` checkpointed (if used).

**Why it matters:** Enables session-to-session continuity, LLM restarts, and retrospective audits.

---

### 10. Lessons Logged or Reused
✅ *"Did the LLM check and/or append to `lessons_learned.jsonl`?"*

- Must consult `.proj_mgmt/lessons_learned.jsonl` on task failure.
- If no relevant entry found, add one post-resolution.
- Includes config bugs, tooling friction, CI issues, LLM missteps.

**Why it matters:** Turns ephemeral failure into shared memory — a cheap way to build intelligence.

---

## 📊 Usage

- The score can be surfaced in dashboards, `test_summaries`, or CI output.
- Total possible: **10 points per task**
- Optional tag: `scorecard_version = "v1"` in JSON output

ROTD doesn't chase perfect tests — it rewards structured progress. These 10 points embody that philosophy.

## ✅ Completion Check

After writing a score entry:
- Log confirmation to terminal:  
  `ROTD scoring complete for task {task_id} (Score: X/10)`
- If score is < 6, suggest a remediation plan or follow-up session.

**Reminder:** *The score is not punitive. It is a project health signal. Use it to prioritize debugging, test stabilization, or documentation sprints.*