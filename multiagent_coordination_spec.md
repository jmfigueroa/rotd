# Development Specification — **ROTD v1.3 “Multi-Agent Safety Pack”**

> Implements full concurrent-agent support, artifact-level locking, task-dependency awareness, priority claiming, review gates, quota tracking, and log hygiene.  
> Scope: CLI changes + coordination directory extensions. No business-logic drift.

---

## 0 — Business Goal

Enable **two-plus Claude Code agents** (Opus/Sonnet/Haiku) to operate simultaneously on the *same* repository without:

* corrupting `.rotd/*` artifacts  
* duplicating work or claiming overlapping tasks  
* overrunning quota or hiding blocked dependencies

---

## 1 — Feature List (Add-Ons to Implement)

* **A1. Artifact-Level File Locks**  
* **A2. Task Dependency Map**  
* **A3. Blocked Status with Reason**  
* **A4. Path-Scoped Source Locks**  
* **A5. Priority-Aware Claiming**  
* **A6. Review Gate (“review → done”)**  
* **A7. Daily `coordination.log` Rotation**  
* **A8. Automated Release Summaries**  
* **A9. Lightweight Quota Tracker**  

*(A0 = existing base registry + agent locks + heartbeats.)*

---

## 2 — Directory & File Additions

```
.rotd/
│
├── coordination/
│   ├── active_work_registry.json      # +blocked, priority, review fields
│   ├── dependency_map.json            # new
│   ├── quota.json                     # new
│   ├── coordination.log               # daily-rotated
│   ├── agent_locks/                   # task-level locks
│   ├── file_locks/                    # path sha1 locks
│   ├── heartbeat/                     # liveness files
│   └── .lock/                         # artifact locks (tasks.lock, registry.lock, …)
└── .lock/                             # top-level locks for standard artifacts
```

---

## 3 — CLI Work-Items

| ID | Command | Behaviour |
|----|---------|-----------|
| **C1** | `rotd coord claim [--any]` | claims highest-priority unclaimed task that has all deps `"done"` & no locks |
| **C2** | `rotd coord release <task>` | marks task `done` (or `review`) & removes lock; auto-summary appended |
| **C3** | `rotd coord approve <task>` | flips `status: review → done`; logs reviewer_id |
| **C4** | `rotd coord msg "<txt>"` | appends to current‐day `coordination.log` |
| **C5** | `rotd coord beat` | updates `<agent>.beat` |
| **C6** | `rotd coord clean-stale` | frees stale locks; rotates log at UTC midnight |
| **C7** | `rotd coord quota --add <tokens>` | adjust `quota.json` counts atomically |
| **C8** | **artifact‐locking** is transparent: all write paths wrapped with `with_lock()` helper |

Implementation language: Rust (existing CLI). Use `fs2::FileExt` for cross-platform flock.

---

## 4 — File-Lock Helper

* `src/fs_ops.rs::with_lock(path, fn)` implements:  
  * create parent `.lock` dir, open lock file, exclusive trylock + 30 s timeout  
  * write metadata JSON (`{"holder": agent_id, "since": ISO}`)  
  * execute critical section, unlock, remove metadata if empty artefact lock

Unit tests: concurrent writer simulation with `std::process::Command`.

---

## 5 — JSON Schemas (Brief)

* **registry** – `status ∈ {unclaimed, claimed, blocked, review, done}`; `priority ∈ {urgent, high, medium, low}`; optional `blocked_reason`, `reviewer_id`.  
* **dependency_map** – flat `{ "task_id": ["dep1","dep2"] }`.  
* **quota** – `{ "tokens_used": 0, "last_reset": ISO, "requests": 0 }`.  
* **coordination.log** – plain lines, rotated daily.  

Schemas added to `/schema/` for CLI validation.

---

## 6 — Migrations

1. CLI `update --yes` will:  
   * create `coordination/` tree, move existing `active_work_registry.json` into it  
   * generate blank `dependency_map.json`, `quota.json`  
   * create `.lock` dirs  
2. Emits copy-paste prompt directing LLM to insert new fields in existing `tasks.jsonl` (priority, deps if known).  
3. Requires human review for first log rotation run.

---

## 7 — Testing Plan

* **T1. Lock-Race Test** – spawn 10 writers, append to `tasks.jsonl`; assert line count == attempts.  
* **T2. Claim Exclusion** – two agents claim; assert different tasks.  
* **T3. Dependency Blocking** – dep map prevents downstream claim.  
* **T4. Block/Unblock** – agent sets `blocked`; cleaner unblocks after reason resolved.  
* **T5. Review Gate** – task cannot reach `done` without `approve`.  
* **T6. Log Rotation** – stub time to midnight; verify new file creation.  
* **T7. Quota Tracker** – concurrent `--add` operations sum correctly, survives crashes.

CI matrix includes Linux & Windows.

---

## 8 — Documentation Updates

* Update `docs/ROTD.md` §“Coordination for Multi-Agent”  
* Add `docs/CLI_COMMANDS.md` entries for new sub-commands  
* Update agent prompts to use `rotd coord claim/release`  
* Provide example `dependency_map.json` and `quota.json`

---

## 9 — Roll-out Milestones

1. **M1** – implement locking helper + C8 wraps (single-agent regression passes)  
2. **M2** – registry schema expansion, claim/release with priority + deps  
3. **M3** – heartbeat cleaner + stale lock recovery  
4. **M4** – path locks & daily log rotation  
5. **M5** – review gate & quota tracker  
6. **M6** – update docs, migration script, release v1.3  
7. **M7** – run dual-agent soak test for 24 h, monitor audit.log

---

## 10 — Out-of-Scope (v1.3)

* Distributed file-system locks (NFS, SMB)  
* Push notifications / WebSocket coordination  
* Full-blown task-graph visualizer (future)  

---

### Deliverables

* Pull request series merged into `main`  
* Version tag `v1.3.0` with changelog  
* Updated schema files and documentation  
* CI green on single and dual-writer test suites

ROTD will then safely scale to **parallel Claude Code agents** without data loss, merge fights, or hidden work overlaps.


# Addendum-B — **Capability-Aware Task Routing Specification**  
*(slots into “ROTD v1.3 Multi-Agent Safety Pack” as section 11)*

---

## 11 Capability-Aware Routing Layer

### 11.1 Problem Statement  
Multiple heterogeneous agents (Opus = planner, Sonnet = generalist, Haiku = test-fixer, etc.) must pull tasks they can complete without overlap. Existing lock/registry prevents duplication but lacks agent–task affinity.

### 11.2 Solution Overview  
* **Opus-only Task Seeding** writes `capability` and `skill_level` fields into every task line.  
* `rotd coord claim` gains optional filters so downstream agents self-select eligible tasks.  
* No other code path altered; registry & locks remain canonical truth.

---

### 11.3 Extended Artifacts

#### 11.3.1 Task JSONL schema (`.rotd/tasks.jsonl`)
```jsonc
{
  "id": "6.2",
  "title": "Keyboard Shortcut Layer",
  "priority": "high",                 // urgent|high|medium|low
  "description": "...",
  "deps": ["4.1","5.2"],
  "capability": "frontend_ts",        // enum: frontend_ts | backend_rust | tests_only | docs | refactor
  "skill_level": "intermediate"       // enum: entry | intermediate | expert
}
```

#### 11.3.2 Registry mirror (`coordination/active_work_registry.json`)  
Same two new keys (`capability`, `skill_level`) added per entry.

---

### 11.4 CLI Extensions

* `rotd coord claim [--capability <cap>] [--skill-level <=entry|<=intermediate|expert]`  
  * Filters registry before lock attempt.  
  * Exit code `0` with empty JSON if no eligible task; agent may back off.

* `rotd coord claim --any` bypasses filters (current behaviour).

* Validation: capability string must match schema; unknown values cause `E_VALIDATION`.

---

### 11.5 Agent Environment Convention

* Each agent sets `ROTD_AGENT_CAPS="cap1,cap2"` and optionally `ROTD_AGENT_SKILL=entry|intermediate|expert`.  
* Startup script calls:  
  ```bash
  rotd coord claim --capability "$cap" --skill-level "<=$ROTD_AGENT_SKILL"
  ```  
  cycling through declared capabilities until a task is returned.

---

### 11.6 Task-Seeding Prompt (Opus)

Opus receives fixed “Task Seeding Prompt” (stored at `docs/prompts_seed.md`) with explicit instructions to:

1. Read `development_spec.md` (+ any design docs).  
2. Output **JSONL only**, using fields defined above.  
3. Honour dependency and priority logic.  
4. Pipe directly into:  
   ```bash
   rotd agent seed-tasks --stdin --validate
   ```

---

### 11.7 Migration Steps

1. Update JSON schemas in `/schema/*.json`.  
2. Add filter logic to `src/coord.rs::cmd_claim`.  
3. Document new env vars & flags in `docs/CLI_COMMANDS.md`.  
4. Provide sample agent bootstrap script in `examples/agent_boot.sh`.  
5. Bump CLI version → `v1.3.1`.

---

### 11.8 Non-Goals

* Automated capability negotiation or learning.  
* Central scheduler process (can be built later).  
* Dynamic capability change mid-session.

---

**Outcome**  
Agents claim tasks that match their skill set, ensuring efficient parallel development while remaining fully ROTD-compliant and lock-safe.  