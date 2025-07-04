# ROTD Primer Strategy — Scalable Machine-Readable Project Orientation

## Goals

- **Fast Orientation:** Let agents understand project scope, roles, structure, and priorities quickly.
- **LLM-Consumable:** JSON or JSONC format optimized for partial loading and safe parsing.
- **Scoped:** Break down by repo and optionally by major directory, not a giant monolith.
- **Auto-indexable:** Allows ROTD tooling to find and rank candidate files per task/domain.

---

## 1. File Structure

**Project Root:**
- `.rotd/primer.jsonc` – High-level project primer: purpose, domain, stack, major components.

**Subdirectories (optional):**
- `.rotd/primer_<dir>.jsonc` (e.g., `primer_api.jsonc`, `primer_ui.jsonc`) – More focused primers for complex areas.

---

## 2. Format Design (`.rotd/primer*.jsonc`)

Use JSONC to support inline explanation where helpful.

```jsonc
{
  "name": "Heimdall Routing Engine",
  "scope": "core/heimdall",
  "description": "Routes natural language inputs to the appropriate backend service or plugin using embedding similarity, rules, and optional heuristics.",
  "status": "active",
  "language": "Rust",
  "entry_points": ["main.rs", "lib.rs"],
  "test_dirs": ["tests/", "integration_tests/"],
  "dependencies": [
    "tokio", "serde", "openai-rs"
  ],
  "known_issues": [
    "Overlaps with CRDT router on plugin dispatch",
    "Embedding scoring sometimes stale due to async cache"
  ],
  "key_concepts": [
    "router protocol", "plugin slot binding", "embedding inference", "query normalization"
  ],
  "preferred_agents": [
    "Opus", "Sonnet"
  ],
  "suggested_starting_points": [
    "Start in `lib.rs` for plugin registration system",
    "See `score.rs` for the routing algorithm"
  ]
}
```

### Recommended Fields
- `name`, `scope`, `description`, `language`, `entry_points`, `test_dirs`
- `dependencies` – helps with LLM context priming
- `known_issues` – get ahead of confusion
- `key_concepts` – semantic compression
- `preferred_agents` – optional, helps with capability routing
- `suggested_starting_points` – good when there's ambiguity

---

## 3. Best Practices

- **Don't try to compress the whole codebase.** Instead, aim to equip agents to ask good questions.
- Use **one `primer.jsonc` per logical domain** only when it's truly isolated.
- Keep primers under ~200 lines; better to split than bloat.
- Use **inline comments (`//`)** in JSONC to assist human reviewers.

---

## 4. ROTD Integration

- `rotd check` can warn if no `.rotd/primer.jsonc` exists.
- Primers can be used during task creation and buckling to rank starting files.
- Use `rotd agent parse-primer` (TBD CLI command) to feed agent memory.

---

## 5. Tooling Ideas

- Add a `rotd primer init` command that scans the directory tree and asks you guided questions per directory.
- Add support for partial diffing of primers as dependencies shift.

---

## 6. Optional Advanced Add-ons

- Add `primer_score` for project health: e.g., completeness of entry_points, presence of test_dirs.
- `primer.tags` for cross-repo topic linking.
- Version the primer with a `primer_version` field for migration support.

---

## Summary

Use `.rotd/primer*.jsonc` as machine-readable, scoped, semantic entry points for agents. Keep them small, modular, and loaded with context that helps agents reason—*not just navigate*. This isn't a replacement for READMEs, it's a precise tool for LLM task bootstrapping, with structure, not sprawl.