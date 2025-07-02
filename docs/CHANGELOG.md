# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Complete ROTD CLI utility with agent and human modes
- Agent-aware prompts for LLM workflows
- JSON schema validation for all ROTD artifacts
- Comprehensive test suite with integration tests
- CI/CD pipeline with automated releases
- Cross-platform binary builds (Linux, macOS, Windows)
- Shell completion support for bash, zsh, fish, and PowerShell
- Audit logging for all ROTD operations
- PSS (Progress Scoring System) integration
- Coverage ratchet mechanism
- Dry-run mode for safe operations
- Comprehensive documentation and examples

### Changed
- Restructured project for dual CLI/manual operation
- Enhanced prompts with CLI-specific instructions
- Improved error handling and validation
- Updated documentation structure

### Fixed
- Schema validation edge cases
- File operation safety and concurrency
- Cross-platform compatibility issues

## [0.1.0] - 2025-07-02

### Added
- Initial ROTD methodology documentation
- Basic Python PSS scoring script
- JSON schemas for validation
- Example ROTD artifacts
- Manual workflow prompts

[Unreleased]: https://github.com/jmfigueroa/rotd/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/jmfigueroa/rotd/releases/tag/v0.1.0