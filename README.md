# Runtime-Oriented Test Discipline (ROTD)

> Test-driven development optimized for LLM workflows, where runtime validation is the single source of truth.

## Why create this?

I needed a way to ensure continuity and effective code output across multiple sessions (even with Claude Max) and with various Agents. It's a struggle to get *one* LLM to perform well in isolation for complex tasks, requiring context retention, consistent task boundaries, and verifiable progress across time. This is doubly hard when trying to orchestrate multiple concurrent agents. This is my attempt to mitigate those problems by enabling agents to inherit context reliably, validate outcomes through runtime tests, and avoid redundant effort, all while keeping the overall system focused, aligned, and durable.

## Quick Start

### Option 1: CLI Installation (Recommended)
```bash
# Install ROTD CLI
curl -sSL https://raw.githubusercontent.com/jmfigueroa/rotd/main/scripts/install.sh | bash

# Initialize project
rotd init

# Check compliance
rotd check
```

### Option 2: Manual Setup
1. **Initialize** your project with a `.rotd/` directory
2. **Track tasks** in `.rotd/tasks.jsonl` 
3. **Write tests** before marking tasks complete
4. **Score progress** with the PSS system
5. **Learn from failures** in `.rotd/lessons_learned.jsonl`

## Philosophy

Traditional TDD: "Write the failing test first."  
**ROTD**: "Explore and implement, but nothing is done until tests prove it works."

ROTD is designed for LLM-assisted development where:
- Exploration and rapid iteration are encouraged
- Runtime validation is the ultimate truth
- Progress is measured systematically
- Failures become shared memory

## Core Files

- **[docs/ROTD.md](./docs/ROTD.md)** - Complete methodology documentation
- **[examples/](./examples/)** - Annotated JSON examples from real projects
- **[schema/](./schema/)** - JSON schemas for validation
- **[scripts/pss_score.py](./scripts/pss_score.py)** - Portable PSS scoring script
- **[docs/prompts.md](./docs/prompts.md)** - LLM prompts for CLI-enabled and manual workflows
- **[src/](./src/)** - Rust CLI utility source code
- **[scripts/install.sh](./scripts/install.sh)** - One-line installation script
- **[docs/README_CLI.md](./docs/README_CLI.md)** - Complete CLI documentation
- **[docs/AGENT_USAGE.md](./docs/AGENT_USAGE.md)** - Quick reference for LLM agents

## Key Principles

1. **Test-Driven**: Every feature needs tests
2. **Runtime Truth**: 100% tests must pass before marking complete
3. **Clean Code**: No TODOs/stubs on main branch
4. **Systematic Progress**: Track everything in structured artifacts
5. **Continuous Learning**: Log failures for future reference

## Artifacts Structure

```
.rotd/                       # ROTD directory (hidden)
├── tasks.jsonl              # Append-only task log
├── test_summaries/          # Proof of completion
├── task_history/            # Per-task change history
├── lessons_learned.jsonl    # Reusable failure patterns
├── pss_scores.jsonl         # Progress scoring results
├── session_state.json       # Delta prompting state
├── coverage_history.json    # Adaptive coverage tracking
├── audit.log                # Rule violations
└── config.jsonc             # ROTD configuration
```

## Progress Scoring System (PSS)

ROTD includes a 10-point scoring system evaluating:

1. **Execution Sanity (1-3)**: LLM engagement, compilation, implementation
2. **Testing Discipline (4-6)**: Tests written/passing, coverage quality  
3. **Cleanup Discipline (7-8)**: Stub-free code, documentation
4. **Historical Continuity (9-10)**: Artifacts maintained, lessons logged

Score tasks with: `rotd score <task_id>` (CLI) or `python scripts/pss_score.py <task_id>` (manual)

## For LLM Agents

ROTD provides structure without rigidity with **two operational modes**:

### CLI-Enabled Mode (Recommended)
- **Use CLI commands**: `rotd agent update-task`, `rotd check`, `rotd show-lessons`
- **Automated validation**: Schema enforcement and audit logging
- **Structured operations**: JSON input/output for programmatic use
- **Safety features**: Dry-run mode and validation checks

### Manual Mode (Fallback)  
- **Direct file editing**: When CLI is unavailable
- **Manual validation**: Self-managed artifact integrity
- **Compatible**: Works with existing ROTD projects

**Key Benefits:**
- **Explore freely** during implementation
- **Test thoroughly** before claiming completion  
- **Score systematically** for project health
- **Learn persistently** from failures

The methodology adapts to LLM workflows while maintaining engineering discipline.

## For Human Developers

ROTD complements traditional development:
- Preserves TDD benefits with LLM flexibility
- Creates auditable progress tracking
- Builds institutional memory through lessons learned
- Provides objective quality metrics

Perfect for hybrid human-AI development teams.

---

## Getting Started

### Quick Start with CLI
```bash
# Install CLI
curl -sSL https://raw.githubusercontent.com/jmfigueroa/rotd/main/scripts/install.sh | bash

# Initialize project
rotd init

# Use CLI-enabled prompts from docs/prompts.md
```

### Manual Setup
```bash
# Create directory structure manually
mkdir -p .rotd/test_summaries
# Copy files from examples/ directory
# Use manual prompts from docs/prompts.md
```

**Complete Documentation**: Read [docs/ROTD.md](./docs/ROTD.md) for methodology details and [docs/README_CLI.md](./docs/README_CLI.md) for CLI usage.

## 🔄 Staying Updated

ROTD methodology evolves with new features and improvements. Keep your projects current:

### Check for Updates
```bash
# Check if updates are available
rotd update --check

# See current version
rotd version
```

### Apply Updates
```bash
# Apply latest methodology updates
rotd update --yes

# After update, use the provided LLM prompt to migrate your project
# The CLI will show you exactly what to copy-paste to your LLM
```

### Update Process
1. **Pull Updates**: ROTD CLI downloads latest methodology and schemas
2. **Migration Guidance**: CLI provides copy-pastable prompts for your LLM
3. **Apply Changes**: LLM migrates your project (adds priority fields, new workflows, etc.)
4. **Verify**: Run `rotd validate --all --strict` to ensure compliance

**New in v1.2.0:**
- Task prioritization system (urgent/high/medium/low/deferred)
- Periodic review process for project health
- Enhanced validation and update automation

See [docs/ROTD_UPDATE_PROTOCOL.md](./docs/ROTD_UPDATE_PROTOCOL.md) for complete update methodology.

## 🏗️ Repository Structure

```
rotd/
├── 📄 Essential Files (Root)
│   ├── README.md             # Project overview and quick start
│   ├── Cargo.toml            # Rust project configuration  
│   ├── LICENSE               # MIT license
│   ├── Makefile              # Build and development commands
│   ├── rustfmt.toml          # Code formatting rules
│   └── .gitignore            # Version control exclusions
│
├── 📚 Documentation
│   └── docs/
│       ├── ROTD.md           # Complete methodology documentation
│       ├── README_CLI.md     # CLI utility documentation
│       ├── AGENT_USAGE.md    # Quick reference for LLM agents
│       ├── CLI_COMMANDS.md   # Command reference and examples
│       ├── prompts.md        # LLM prompts (CLI-enabled & manual)
│       ├── CONTRIBUTING.md   # Contribution guidelines
│       └── CHANGELOG.md      # Version history
│
├── 🛠️ CLI Source Code
│   └── src/
│       ├── main.rs           # CLI entry point and argument parsing
│       ├── agent.rs          # Agent mode commands (JSON I/O)
│       ├── human.rs          # Human mode commands (interactive)
│       ├── schema.rs         # Data structures and validation
│       ├── fs_ops.rs         # File operations and safety
│       ├── pss.rs            # Progress Scoring System
│       ├── audit.rs          # Audit logging
│       └── common.rs         # Shared utilities
│
├── 🚀 Scripts & Tools
│   └── scripts/
│       ├── install.sh        # One-line installation script
│       └── pss_score.py      # Portable Python scoring script
│
├── 📊 Examples & Schemas
│   ├── examples/             # Real ROTD artifacts from projects
│   │   ├── tasks.jsonl       # Example task entries
│   │   ├── pss_score.jsonl   # Example PSS scores
│   │   ├── lessons_learned.jsonl # Example lessons
│   │   └── test_summary.json # Example test results
│   └── schema/               # JSON schemas for validation
│       ├── task.schema.json  # Task entry schema
│       └── pss_score.schema.json # PSS score schema
│
└── 🧪 Testing & CI
    ├── tests/
    │   └── integration_test.rs # CLI integration tests
    └── .github/workflows/
        ├── ci.yml            # Continuous integration
        └── release.yml       # Automated releases
```

### 📁 Key Directories

- **`docs/`**: All documentation and guides
- **`src/`**: Rust CLI source code with modular architecture  
- **`scripts/`**: Installation and utility scripts
- **`examples/`**: Real-world ROTD artifacts for reference
- **`schema/`**: JSON schemas for validation and tooling
- **`tests/`**: Integration tests and test utilities
- **`.github/`**: CI/CD pipelines and GitHub configuration

### 🎯 Design Philosophy

- **Clean root**: Only essential build/config files at project root
- **Organized by purpose**: Documentation, source code, scripts, examples in separate directories
- **Easy navigation**: Clear directory structure with descriptive names
- **Maintainable**: Logical grouping reduces cognitive load