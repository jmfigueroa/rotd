# Git Commit Rules for ROTD

> Short, meaningful commit messages with detailed notes in git notes

## Commit Message Format

ROTD follows a simplified version of conventional commits with detailed information stored in git notes:

```
<type>: <short summary>
```

### Commit Header

The commit header is a single line that includes the type and short summary:

- **Type**: Describes the kind of change (feat, fix, docs, etc.)
- **Summary**: Brief description (50 chars max)
- **Examples**: 
  - `feat: Add Buckle Mode recovery protocol`
  - `fix: Correct task status validation`

### Git Notes

Detailed information about the changes is stored in git notes:

- Contains the motivation and details of the change
- Can use bullet points (each line starting with -)
- Should answer "why" not just "what"
- Formatted in markdown for readability

### Example Commit and Note

**Commit message:**
```
feat: Add Buckle Mode recovery protocol
```

**Git note:**
```markdown
# Buckle Mode Recovery Protocol

Implements ROTD Buckle Mode for recovery from compilation and artifact integrity failures.

## Changes Made

- Add Buckle Mode documentation and CLI commands
- Create audit rule for automatic Buckle Mode triggering
- Update related documentation files
- Add CLI implementation scaffold
- Emphasize task tracking integrity requirements

## Impact

Provides systematic recovery mechanism for ROTD projects when compilation or artifact integrity is compromised.
```

## Commit Types

| Type | Description |
|------|-------------|
| `feat` | New feature or functionality |
| `fix` | Bug fix |
| `docs` | Documentation changes only |
| `style` | Formatting, whitespace (no code change) |
| `refactor` | Code restructuring (no feature change) |
| `perf` | Performance improvements |
| `test` | Adding/updating tests |
| `build` | Build system changes |
| `ci` | CI configuration changes |
| `chore` | Maintenance tasks |

## Best Practices

1. **Be concise**: Commit message should be 50 chars or less
2. **Be specific**: Describe what the change does clearly
3. **Be complete**: Include all relevant details in the git note
4. **Be consistent**: Follow the format for all commits
5. **Be atomic**: One logical change per commit

## For ROTD Projects

When committing to ROTD-managed projects, include a reference to the task ID in the git note:

**Commit message:**
```
feat: Implement WebSocket authentication
```

**Git note:**
```markdown
# WebSocket Authentication Implementation

## Changes Made

- Add token validation
- Create session management
- Update client connection handling

## ROTD Task Reference

Task-ID: 5.2

## Test Coverage

All changes covered by integration tests in `tests/websocket_auth_test.rs`.
```

The Task-ID reference in the git note helps link commits to their associated ROTD tasks and test summaries.

## Git Notes Commands

To work with git notes in ROTD projects:

### Adding Notes
```bash
# Add a note to the current commit
git notes add -m "# Detailed explanation

## Changes Made
- List of changes
- More details

## Task Reference
Task-ID: X.Y"

# Add a note to a specific commit
git notes add <commit-hash> -m "Detailed note content"
```

### Viewing Notes
```bash
# Show notes for current commit
git notes show

# Show notes for specific commit
git notes show <commit-hash>

# Show log with notes
git log --show-notes
```

### Editing Notes
```bash
# Edit existing note
git notes edit

# Edit note for specific commit
git notes edit <commit-hash>
```

## Git Hooks

ROTD includes a pre-commit hook template that helps enforce these rules. Install it with:

```bash
cp .git-hooks/pre-commit .git/hooks/
chmod +x .git/hooks/pre-commit
```

The hook will:
- Check commit message format (single line)
- Verify git notes contain Task-ID references
- Ensure all tasks are properly updated in tasks.jsonl