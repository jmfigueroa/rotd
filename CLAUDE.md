# ROTD Project Instructions

## Core Commands
- Build: `cargo build --release`
- Test: `cargo test`
- Run: `cargo run -- [command]`
- Check ROTD compliance: `cargo run -- check`

## ROTD Workflow
IMPORTANT: This project uses ROTD (Runtime-Oriented Test Discipline). Always:
1. Update tasks in `.rotd/tasks.jsonl` as you work
2. Create test summaries for completed tasks
3. Run `cargo run -- check` to verify compliance

### Key ROTD Commands
- Initialize: `rotd init`
- Update task: `echo '{"id":"X.Y","status":"complete"}' | rotd agent update-task --timestamp`
- Add test summary: `rotd agent append-summary --file test_summary.json`
- Check health: `rotd --agent check`

## Code Style
- Use clear, descriptive function names
- Add comments for complex logic
- Follow Rust conventions and idioms
- Run `cargo fmt` before committing

## Testing
- Write tests for new functionality
- Ensure all tests pass before marking tasks complete
- Integration tests are in `tests/`

## Project Structure
- `src/agent.rs` - Agent mode commands (JSON output)
- `src/human.rs` - Human-friendly commands
- `src/schema.rs` - Data structures
- `.rotd/` - ROTD tracking data

## IMPORTANT
- Never modify `.rotd/` files directly - use CLI commands
- Always create test summaries for completed tasks
- The `fix` parameter in checks will auto-create missing files