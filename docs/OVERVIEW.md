# ROTD Project Overview

> Quick orientation guide for contributors and users

## 🎯 What is ROTD?

**Runtime-Oriented Test Discipline (ROTD)** is a test-driven development methodology optimized for LLM workflows, where runtime validation is the single source of truth.

### Key Innovations

- **LLM-first design**: Structured for AI-assisted development
- **Runtime validation**: Tests are the ultimate truth
- **Agent-aware tooling**: CLI supports both human and LLM users
- **Systematic progress tracking**: PSS scoring and audit logging
- **Persistent memory**: Lessons learned across sessions

## 🏗️ Project Components

### 1. Methodology (docs/ROTD.md)
The complete ROTD framework including:
- Core principles and philosophy
- Artifact structure and schemas
- Task lifecycle and enforcement rules
- Progress Scoring System (PSS)
- Efficiency optimizations for LLM workflows

### 2. CLI Tool (src/)
Rust-based command-line interface with:
- **Human mode**: Interactive, colored output, helpful prompts
- **Agent mode**: JSON I/O, strict validation, programmatic use
- **Safety features**: Dry-run mode, schema validation, audit logging
- **Cross-platform**: Linux, macOS, Windows support

### 3. Documentation (docs/)
Comprehensive guides including:
- **prompts.md**: LLM prompts for CLI-enabled and manual workflows
- **README_CLI.md**: Complete CLI documentation
- **AGENT_USAGE.md**: Quick reference for LLM agents
- **CLI_COMMANDS.md**: Command reference with examples
- **CONTRIBUTING.md**: Developer guidelines

### 4. Examples & Schemas (examples/, schema/)
Real-world artifacts and validation:
- JSON schemas for all ROTD data structures
- Example artifacts from actual projects
- Validation tools and reference implementations

## 🔄 Typical Workflows

### For LLM Agents (Recommended)
```bash
# Initialize project
rotd init

# Update task status
echo '{"id":"X.Y","status":"complete"}' | rotd agent update-task --timestamp --pss

# Log test results
rotd agent append-summary --file test_summaries/X.Y.json

# Record lessons learned
echo '{"id":"lesson-id","diagnosis":"...","remediation":"..."}' | rotd agent log-lesson

# Check health
rotd check
```

### For Human Developers
```bash
# Initialize and check health
rotd init
rotd check --verbose

# Review progress
rotd show-task X.Y --verbose
rotd show-lessons --tag=recent
rotd score X.Y --format summary

# Monitor activity
rotd show-audit --limit=20
```

### For Manual Workflows (Fallback)
```bash
# Create structure manually
mkdir -p .rotd/test_summaries

# Use Python scoring
python scripts/pss_score.py X.Y

# Follow manual prompts from docs/prompts.md
```

## 🎨 Design Philosophy

### Clean Architecture
- **Separation of concerns**: CLI, methodology, documentation
- **Modular design**: Independent components with clear interfaces
- **Extensible**: Easy to add new commands and features

### User Experience
- **Human-friendly defaults**: Colored output, helpful messages
- **Agent-optimized modes**: JSON I/O, strict validation
- **Safety first**: Dry-run mode, validation, rollback capabilities

### Developer Experience
- **Clear documentation**: Comprehensive guides and examples
- **Easy contribution**: Well-defined processes and standards
- **Quality assurance**: CI/CD, testing, linting

## 🚀 Getting Started

### For Users
1. **Install**: `curl -sSL https://raw.githubusercontent.com/jmfigueroa/rotd/main/scripts/install.sh | bash`
2. **Initialize**: `rotd init`
3. **Learn**: Read [docs/ROTD.md](./ROTD.md) for methodology details

### For Contributors
1. **Clone**: `git clone https://github.com/jmfigueroa/rotd.git`
2. **Build**: `cargo build`
3. **Test**: `cargo test`
4. **Read**: [docs/CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines

### For LLM Integration
1. **Install CLI**: Follow installation guide
2. **Use prompts**: Reference [docs/prompts.md](./prompts.md)
3. **Agent commands**: See [docs/AGENT_USAGE.md](./AGENT_USAGE.md)

## 📊 Project Status

- **Version**: 0.1.0 (initial release)
- **Language**: Rust (CLI), Python (scripts)
- **License**: MIT
- **Platform**: Cross-platform (Linux, macOS, Windows)
- **CI/CD**: GitHub Actions with automated testing and releases

## 🤝 Community

- **Issues**: Bug reports and feature requests
- **Discussions**: Questions and ideas
- **Pull Requests**: Code contributions welcome
- **Documentation**: Help improve guides and examples

This project bridges the gap between traditional software development practices and modern AI-assisted workflows, providing structure without sacrificing the flexibility that makes LLM collaboration effective.