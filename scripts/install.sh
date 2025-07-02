#!/bin/bash

# ROTD CLI Installation Script
# Usage: curl -sSL https://raw.githubusercontent.com/jmfigueroa/rotd/main/scripts/install.sh | bash

set -e

REPO="jmfigueroa/rotd"
BINARY_NAME="rotd"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${CYAN}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if cargo is installed
check_cargo() {
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo is not installed. Please install Rust first:"
        log_info "Visit https://rustup.rs/ for installation instructions"
        exit 1
    fi
    log_info "Cargo found: $(cargo --version)"
}

# Check if git is installed
check_git() {
    if ! command -v git &> /dev/null; then
        log_error "Git is not installed. Please install Git first."
        exit 1
    fi
    log_info "Git found: $(git --version)"
}

# Install ROTD CLI
install_rotd() {
    log_info "Installing ROTD CLI from GitHub repository..."
    
    if cargo install --git "https://github.com/${REPO}" --branch main; then
        log_success "ROTD CLI installed successfully!"
    else
        log_error "Installation failed. Please check the error messages above."
        exit 1
    fi
}

# Verify installation
verify_installation() {
    if command -v "$BINARY_NAME" &> /dev/null; then
        local version
        version=$("$BINARY_NAME" --version)
        log_success "Installation verified: $version"
        return 0
    else
        log_error "Installation verification failed. '$BINARY_NAME' command not found."
        log_warn "Make sure ~/.cargo/bin is in your PATH:"
        log_info "export PATH=\"\$HOME/.cargo/bin:\$PATH\""
        return 1
    fi
}

# Add cargo bin to PATH if needed
check_path() {
    if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
        log_warn "~/.cargo/bin is not in your PATH"
        log_info "Add this to your shell configuration file (.bashrc, .zshrc, etc.):"
        log_info "export PATH=\"\$HOME/.cargo/bin:\$PATH\""
        echo
        log_info "For this session, run:"
        log_info "export PATH=\"\$HOME/.cargo/bin:\$PATH\""
    fi
}

# Show usage examples
show_examples() {
    echo
    log_info "Quick start examples:"
    echo "  Initialize ROTD in a project:    $BINARY_NAME init"
    echo "  Check project health:            $BINARY_NAME check"
    echo "  Score all tasks:                 $BINARY_NAME score"
    echo "  Generate shell completions:      $BINARY_NAME completions bash"
    echo
    log_info "For full documentation, see: https://github.com/$REPO"
}

# Main installation flow
main() {
    echo
    log_info "🚀 ROTD CLI Installation Script"
    log_info "Installing from: https://github.com/$REPO"
    echo

    check_cargo
    check_git
    install_rotd
    
    echo
    if verify_installation; then
        check_path
        show_examples
        echo
        log_success "🎉 ROTD CLI is ready to use!"
    else
        log_error "Installation completed but verification failed."
        log_info "Try running 'source ~/.bashrc' or restart your terminal."
        exit 1
    fi
}

# Handle interrupts gracefully
trap 'log_error "Installation interrupted"; exit 1' INT TERM

# Run main function
main "$@"