# Git Commit Rules for ROTD

> Short, meaningful commit messages with detailed notes in the body

## Commit Message Format

ROTD follows a simplified version of conventional commits with a focus on clarity and brevity:

```
<type>: <short summary>

<detailed description>
```

### Commit Header

The first line is the commit header, which includes the type and short summary:

- **Type**: Describes the kind of change (feat, fix, docs, etc.)
- **Summary**: Brief description (50 chars max)
- **Examples**: 
  - `feat: Add Buckle Mode recovery protocol`
  - `fix: Correct task status validation`

### Commit Body

The commit body provides detailed notes about the changes:

- Separated from header by a blank line
- Explains the motivation and details of the change
- Can use bullet points (each line starting with -)
- Should answer "why" not just "what"

### Example Commit Message

```
feat: Add Buckle Mode recovery protocol

Implements ROTD Buckle Mode for recovery from compilation and artifact integrity failures.

- Add Buckle Mode documentation and CLI commands
- Create audit rule for automatic Buckle Mode triggering
- Update related documentation files
- Add CLI implementation scaffold
- Emphasize task tracking integrity requirements
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

1. **Be concise**: First line should be 50 chars or less
2. **Be specific**: Describe what the change does clearly
3. **Be complete**: Include all relevant details in the body
4. **Be consistent**: Follow the format for all commits
5. **Be atomic**: One logical change per commit

## For ROTD Projects

When committing to ROTD-managed projects, include a reference to the task ID:

```
feat: Implement WebSocket authentication

- Add token validation
- Create session management
- Update client connection handling

Task-ID: 5.2
```

The Task-ID reference helps link commits to their associated ROTD tasks and test summaries.

## Git Hooks

ROTD includes a pre-commit hook template that helps enforce these rules. Install it with:

```bash
cp .git-hooks/pre-commit .git/hooks/
chmod +x .git/hooks/pre-commit
```

The hook will:
- Check commit message format
- Verify Task-ID references
- Ensure all tasks are properly updated in tasks.jsonl