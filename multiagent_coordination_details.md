# Multi-Agent Coordination Layer for ROTD  
*(non-overlapping task execution, locking, and lightweight messaging)*  


## 1 Directory Layout — all inside `.rotd/coordination`

| Path | Purpose |
|------|---------|
| `active_work_registry.json` | Canonical list of **claimable tasks** and who currently owns them |
| `agent_locks/`              | One lock-file per task+agent (`<task_id>.<agent_id>.lock`) |
| `coordination.log`          | Append-only text log for inter-agent messages |
| `heartbeat/`                | `<agent_id>.beat` files; `mtime` = agent liveness |


## 2 Agent Life-Cycle

1. **Startup**  
   - Generate/read persistent `agent_id` (UUID).  
   - Touch `heartbeat/<agent_id>.beat` every `T_beat_interval`.  

2. **Claim Work**  
   - Read `active_work_registry.json`.  
   - Choose first `"status":"unclaimed"` task without a lock.  
   - Atomically create `agent_locks/<task_id>.<agent_id>.lock`.  
   - Update registry entry → `"status":"claimed"`, `"claimed_by": agent_id`, `"claimed_at": ISO`.  

3. **Execute Task**  
   - Standard ROTD flow: update `.rotd/tasks.jsonl`, run tests, create summary, score.  
   - Emit progress lines to `coordination.log`:  
     `[ISO] <agent_id> ▶ progress task 4.2 45%`  

4. **Release / Complete**  
   - Mark registry `"status":"done"`, `"completed_at": ISO`.  
   - Delete lock file.  
   - Log completion line.  

5. **Stale-Lock Recovery**  
   - Cleaner (cron or each agent on startup):  
     - For every file in `agent_locks/`, parse `<agent_id>`; check `heartbeat/<agent_id>.beat`.  
     - If beat older than `T_timeout` (15 min default):  
       - Remove lock, set registry back to `"unclaimed"`.  
       - Log: `[ISO] coordinator freed stale lock task 4.2 (agent dead-uuid)`.


## 3 File Formats

```jsonc
// active_work_registry.json
{
  "tasks": [
    {
      "id": "4.2",
      "title": "Preamble Editor Modal",
      "status": "unclaimed" | "claimed" | "done",
      "priority": "high" | "medium" | "low",
      "claimed_by": "agent-uuid",     // null when unclaimed
      "claimed_at": "2025-07-06T12:34Z",
      "completed_at": null
    }
  ]
}
```

Lock file (optional content):
```json
{ "task_id":"4.2","agent_id":"agent-uuid","created_at":"ISO" }
```


## 4 Proposed CLI Extensions

| Command | Purpose |
|---------|---------|
| `rotd coord claim`                  | Atomically claim next unclaimed task (prints JSON) |
| `rotd coord release <task_id>`      | Mark done + drop lock |
| `rotd coord ls`                     | Show registry snapshot |
| `rotd coord msg "<text>"`           | Append to `coordination.log` |
| `rotd coord beat`                   | Touch own heartbeat file |
| `rotd coord clean-stale`            | Run stale-lock sweep |


## 5 Timeout / Cleaner Parameters

| Name | Default | Notes |
|------|---------|-------|
| `T_beat_interval` | 60 s  | Agent heartbeat cadence |
| `T_timeout`       | 900 s | 15 min inactivity ⇒ stale |
| Cleaner schedule  | 5 min | Low overhead sweep |


## 6 Communication Log Convention

Plain lines, one per event:

```
[2025-07-06T12:34:01Z] agent-a ▶ claimed task 4.2
[2025-07-06T12:55:30Z] agent-a ▶ completed task 4.2; PSS=9
[2025-07-06T12:56:10Z] agent-b ▶ blocked: waiting on 4.2 summary
```

Agents **tail read-only** to coordinate.


## 7 Non-Overlap Guarantee

- Task status **unclaimed → claimed → done**  
- Lock file + registry = canonical truth  
- Atomic `open(O_EXCL)` prevents dual claim


## 8 ROTD Integration

- Registry supplements, **does not replace**, `.rotd/tasks.jsonl`
- `rotd check` still enforces artifact integrity
- Lessons, audit, coverage stay unchanged



## Precision-First Add-Ons (without table markup)

Below are lean, high-impact coordination features you may layer onto the basic lock/registry design. Each addresses a concrete conflict pattern while avoiding bloat.


1. **Dependency Map (`coordination/dependency_map.json`)**  
   *Problem* – agents can claim tasks whose prerequisites are unfinished.  
   *Solution* – a small JSON dictionary lists task → prerequisite IDs.  
   *Effect* – `rotd coord claim` refuses a task if any prerequisite’s registry status ≠ “done”.  
   Example snippet:  
   ```jsonc
   {
     "4.3": ["4.2"],
     "6.2": ["4.1", "5.2"]
   }
   ```

2. **Blocked Status with Reason**  
   *Problem* – unclear hand-off when a task stalls waiting for review or data.  
   *Solution* – allow `"status":"blocked"` plus `"blocked_reason":"waiting on review"` in the registry.  
   *Effect* – agents and cleaner scripts know why work paused and can auto-unblock if the reason is resolved.

3. **Path-Scoped Locks (file-level)**  
   *Problem* – two agents editing different tasks still touch the same source file.  
   *Solution* – optional `file_locks/sha1(<path>).lock` files created on first write, deleted after commit.  
   *Effect* – zero chance of merge conflicts on hot files; overhead only for contested paths.

4. **Priority-Aware Claiming**  
   *Problem* – critical tasks starve while low-priority work claims cycles.  
   *Solution* – registry already stores `"priority":"high|medium|low"`; claim command picks highest first unless `--any` flag supplied.  
   *Effect* – agents converge on the critical path by default.

5. **Review Gate (review → done)**  
   *Problem* – tasks marked “done” without oversight.  
   *Solution* – introduce `"status":"review"` plus `"reviewer_id":""`. Only `rotd coord approve <task>` can flip to `"done"`.  
   *Effect* – human or QC-agent sign-off before completion, minimal ceremony.

6. **Daily Rotation of `coordination.log`**  
   *Problem* – monolithic log grows unwieldy.  
   *Solution* – at UTC midnight rename to `coordination-YYYY-MM-DD.log`; create new file automatically.  
   *Effect* – keeps logs tailable and diffs small without external tooling.

7. **Automated Release Summaries**  
   *Problem* – next agent lacks quick context.  
   *Solution* – upon `rotd coord release`, capture a one-liner (e.g., `git diff --stat` or brief summary) and append to the log.  
   *Effect* – arriving agent sees what actually changed at a glance.

8. **Lightweight Quota Tracker (`coordination/quota.json`)**  
   *Problem* – multiple agents can burn provider quota unexpectedly.  
   *Solution* – small JSON file holds token / request counts. Each agent updates after heavy LLM calls.  
   *Effect* – agents check budget before large operations, preventing outages.





## Precision-First Add-On — **Artifact-Level File-Locking for Concurrent Agents**

This document extends the “Precision-First Add-Ons” list with an essential safeguard that enables **multiple Claude Code agents** (Opus, Sonnet, or Haiku) to invoke the **ROTD CLI in parallel** without corrupting shared JSON/JSONL artifacts.


## 🔑 Goal  
Guarantee that any **write** to a ROTD file is atomic and exclusive, while keeping **reads** lock-free and fast.


1. Locking Model

* **One lock per mutable artifact** (fine-grained):  
  * `.rotd/.lock/tasks.lock`  
  * `.rotd/.lock/test_summaries.lock`  
  * `.rotd/.lock/pss_scores.lock`  
  * `.rotd/.lock/session_state.lock`  
  * `.rotd/coordination/.lock/registry.lock`  

* Locks are created via the OS **exclusive open**:  
  * Unix → `flock` (`libc::flock(fd, LOCK_EX|LOCK_NB)`)  
  * Windows → `LockFileEx` with `LOCKFILE_EXCLUSIVE_LOCK`

* **Timeout**: if a lock cannot be acquired within *30 s*, CLI exits with code `4` (`E_LOCK_TIMEOUT`).  
  Agents can back-off and retry or escalate to Buckle Mode.


2. Write Procedure (JSONL append)

1. `acquire_lock(".rotd/.lock/tasks.lock")`  
2. `open("+append")`, write **one full line**, `\n`, `fsync`  
3. `release_lock(...)`

**Guarantee** – lines will never interleave; tail remains valid JSONL.


3. Write Procedure (JSON overwrite)

For JSON objects (e.g., `session_state.json`):

1. `acquire_lock(".rotd/.lock/session_state.lock")`  
2. Read file → modify in memory  
3. Write to temp: `session_state.json.tmp`  
4. `fsync` temp → `rename()` to original (atomic)  
5. `release_lock(...)`


4. CLI Changes

* **Transparent**: all existing commands auto-lock; agent scripts unchanged.  
* **Two new flags**  
  * `--lock-wait <sec>` override timeout  
  * `--lock-diagnose` prints holder info (`{"holder":"agent-uuid","since":"ISO"}`)

* **Exit codes**  
  * `4` `E_LOCK_TIMEOUT` – lock not acquired in time  
  * `5` `E_LOCK_STALE` – stale lock cleared automatically, operation retried once

Lock metadata (agent id + timestamp) is stored inside each `.lock` file as a small JSON blob for diagnostic purposes.


5. Interaction with Coordination Registry

* `active_work_registry.json` writes are already behind `coordination/.lock/registry.lock`.  
* File-level locks and task-level locks **complement** each other:  
  * Task lock guarantees non-overlap by *task intent*.  
  * File lock guarantees byte-level safety for shared artifacts.


6. Stale-Lock Handling

* Heartbeat cleaner (see previous add-on) removes **agent locks**.  
* File locks are removed automatically after operation; if CLI crashes, the `.lock` file persists – subsequent CLI detects absent heartbeat and treats it as **stale** (`E_LOCK_STALE`), removes, retries once, then proceeds.


7. Minimal Rust Helper (conceptual)

```rust
fn with_lock<F, P>(path: P, f: F) -> Result<()>
where
    F: FnOnce() -> Result<()>,
    P: AsRef<Path>,
{
    use fs2::FileExt; // cross-platform flock crate
    let lock_path = Path::new(path.as_ref());
    std::fs::create_dir_all(lock_path.parent().unwrap())?;
    let file = OpenOptions::new().read(true).write(true).create(true).open(lock_path)?;
    let start = Instant::now();
    while file.try_lock_exclusive().is_err() {
        if start.elapsed() > Duration::from_secs(30) {
            return Err("E_LOCK_TIMEOUT".into());
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    // run critical section
    let res = f();
    file.unlock()?;
    res
}
```

*(Add metadata write + stale detection in production code.)*


8. Roll-Out Steps

1. Implement helper in `src/fs_ops.rs`, wrap every mutating CLI path.  
2. Add integration test spawning two parallel child processes writing to `tasks.jsonl`.  
3. Release **ROTD v1.3.0** with “concurrent-safe CLI” note.  
4. Update agent prompts to remove any “single-agent” caveat.


#Result

**Guiding principle:** introduce a coordination artifact *only* when it prevents a real, recurring conflict. Start with the base lock/registry; adopt these add-ons incrementally as pain points emerge. 

With artifact-level locks in place, multiple Claude agents can confidently run `rotd` commands concurrently—eliminating race conditions while keeping overhead minimal.  



## Addendum — Multi-Agent Task Seeding & Capability-Aware Execution

This update preserves **Opus as the single “planner”** that explodes work into ROTD tasks, while enabling **any number of downstream agents (Sonnet, Haiku, GPT-4o, etc.)** to claim only the tasks they are best suited to execute.

---

### 1 Task-Seeding Protocol (unchanged root idea)

**Prompt Opus once**:

1. Read `development_spec.md` and any domain docs.  
2. Output **JSONL** lines with required fields (see schema below).  
3. Pipe straight into `rotd agent seed-tasks --stdin --validate`.

```bash
opus_cli --model claude-opus-4 prompt_seed.md \
  | rotd agent seed-tasks --stdin --validate
```

---

### 2 Extended Task Schema

Add two new optional fields (highlighted):

```jsonc
{
  "id": "6.2",
  "title": "Keyboard Shortcut Layer",
  "description": "Global hotkeys with modal awareness",
  "priority": "high",                     // urgent|high|medium|low
  "deps": ["4.1", "5.2"],                 // existing
  "capability": "frontend_ts",            // < NEW
  "skill_level": "intermediate"           // < NEW (entry|intermediate|expert)
}
```

*`capability`* drives **which agent may claim**, while *`skill_level`* can help an orchestrator route tasks to stronger/weaker models.

---

### 3 Registry Update

`active_work_registry.json` gains the same two fields so claim logic can filter quickly:

```jsonc
{
  "id": "6.2",
  "status": "unclaimed",
  "priority": "high",
  "capability": "frontend_ts",
  "skill_level": "intermediate",
  ...
}
```

---

### 4 CLI Enhancements

* `rotd coord claim                                # current behavior`
* `rotd coord claim --capability backend_rust      # filter`
* `rotd coord claim --skill-level <=intermediate   # weaker agent`
* If **both filters** passed, agent may claim; otherwise CLI exits 0 with `"no_eligible_task"`.

---

### 5 Agent Boot Sequence

1. Detect own **capabilities** (hard-coded or via env var:  
   `ROTD_AGENT_CAPS="frontend_ts,tests_only"`).  
2. Run `rotd coord claim --capability frontend_ts`.  
3. If nothing returned, sleep / poll or degrade to another capability.  
4. Proceed with standard lock–work–release cycle.

Haiku could declare `capability=tests_only`, Sonnet may claim `docs` or mid-level backend tasks, while Opus can claim expert or cross-cutting fixes when required.

---

### 6 Optional Routing Orchestrator

If you eventually run a **controller process** (could be another Opus instance) you can:

* Inspect registry + quotas + agent heartbeats.  
* Spawn new Claude/Sonnet processes, passing appropriate `ROTD_AGENT_CAPS`.  
* Kill idle agents once backlog drains.

This keeps manual oversight minimal while still respecting ROTD’s artifact-centric discipline.

---

### 7 Why No In-Memory Lists?

All routing decisions flow through **locked registry + CLI**. No agent stores its own queue, which guarantees accuracy even if one crashes or if tasks are reprioritized by the planner.

---

**Result:**  
A single planning pass by Opus seeds well-described tasks. Any mix of agents can safely self-assign work that matches their declared capabilities, with zero risk of overlap or artifact corruption.  