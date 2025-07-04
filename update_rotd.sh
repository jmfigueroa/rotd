#!/bin/bash

# ROTD Manual Update Script
# Downloads and installs the latest ROTD binary from GitHub releases

set -e

REPO="jmfigueroa/rotd"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="rotd"
TEMP_DIR=$(mktemp -d)
GITHUB_API_URL="https://api.github.com/repos/$REPO/releases/latest"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Detect platform
detect_platform() {
    local os=$(uname -s)
    local arch=$(uname -m)
    
    case "$os" in
        Darwin)
            case "$arch" in
                x86_64) echo "macos-x86_64" ;;
                arm64) echo "macos-arm64" ;;
                *) log_error "Unsupported macOS architecture: $arch"; exit 1 ;;
            esac
            ;;
        Linux)
            case "$arch" in
                x86_64) echo "linux-x86_64" ;;
                aarch64|arm64) echo "linux-arm64" ;;
                *) log_error "Unsupported Linux architecture: $arch"; exit 1 ;;
            esac
            ;;
        MINGW*|MSYS*|CYGWIN*)
            case "$arch" in
                x86_64) echo "windows-x86_64" ;;
                *) log_error "Unsupported Windows architecture: $arch"; exit 1 ;;
            esac
            ;;
        *)
            log_error "Unsupported operating system: $os"
            exit 1
            ;;
    esac
}

# Get current version
get_current_version() {
    if command -v $BINARY_NAME &> /dev/null; then
        $BINARY_NAME --version 2>/dev/null | grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' || echo "unknown"
    else
        echo "not installed"
    fi
}

# Check if running as root (for system-wide installation)
check_permissions() {
    if [[ "$INSTALL_DIR" == "/usr/local/bin" ]] && [[ $EUID -ne 0 ]]; then
        log_warning "Installing to $INSTALL_DIR requires sudo privileges"
        return 1
    fi
    return 0
}

# Download and extract binary
download_and_install() {
    local platform="$1"
    local version="$2"
    local download_url="$3"
    local asset_name="$4"
    
    log_info "Downloading $asset_name..."
    
    # Download the asset
    if ! curl -L -o "$TEMP_DIR/$asset_name" "$download_url"; then
        log_error "Failed to download $asset_name"
        exit 1
    fi
    
    # Extract based on file type
    cd "$TEMP_DIR"
    if [[ "$asset_name" == *.tar.gz ]]; then
        log_info "Extracting tar.gz archive..."
        tar -xzf "$asset_name"
    elif [[ "$asset_name" == *.zip ]]; then
        log_info "Extracting zip archive..."
        unzip -q "$asset_name"
    else
        log_error "Unknown archive format: $asset_name"
        exit 1
    fi
    
    # Find the binary
    local binary_path=""
    if [[ -f "$BINARY_NAME" ]]; then
        binary_path="$BINARY_NAME"
    elif [[ -f "$BINARY_NAME.exe" ]]; then
        binary_path="$BINARY_NAME.exe"
    else
        # Look for binary in subdirectories
        binary_path=$(find . -name "$BINARY_NAME" -o -name "$BINARY_NAME.exe" | head -1)
        if [[ -z "$binary_path" ]]; then
            log_error "Binary not found in archive"
            exit 1
        fi
    fi
    
    # Make binary executable
    chmod +x "$binary_path"
    
    # Install binary
    log_info "Installing to $INSTALL_DIR..."
    
    # Create install directory if it doesn't exist
    if [[ ! -d "$INSTALL_DIR" ]]; then
        mkdir -p "$INSTALL_DIR" || {
            log_error "Failed to create install directory: $INSTALL_DIR"
            exit 1
        }
    fi
    
    # Copy binary to install directory
    if ! cp "$binary_path" "$INSTALL_DIR/$BINARY_NAME"; then
        log_error "Failed to install binary to $INSTALL_DIR"
        exit 1
    fi
    
    log_success "Successfully installed $BINARY_NAME $version to $INSTALL_DIR"
}

# Main update function
main() {
    log_info "ROTD Manual Update Script"
    log_info "Repository: https://github.com/$REPO"
    echo
    
    # Check current version
    local current_version=$(get_current_version)
    log_info "Current version: $current_version"
    
    # Detect platform
    local platform=$(detect_platform)
    log_info "Detected platform: $platform"
    
    # Get latest release info
    log_info "Fetching latest release information..."
    local release_info
    if ! release_info=$(curl -s "$GITHUB_API_URL"); then
        log_error "Failed to fetch release information from GitHub API"
        exit 1
    fi
    
    # Parse release info
    local latest_version=$(echo "$release_info" | grep -o '"tag_name": *"[^"]*"' | grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+')
    local release_name=$(echo "$release_info" | grep -o '"name": *"[^"]*"' | head -1 | sed 's/"name": *"//;s/"//')
    
    if [[ -z "$latest_version" ]]; then
        log_error "Could not parse latest version from GitHub API"
        exit 1
    fi
    
    log_info "Latest version: $latest_version"
    log_info "Release: $release_name"
    
    # Check if update is needed
    if [[ "$current_version" == "$latest_version" ]]; then
        log_success "Already up to date!"
        exit 0
    fi
    
    # Find matching asset
    local asset_name=""
    local download_url=""
    
    # Look for platform-specific asset
    if [[ "$platform" == "macos"* ]]; then
        # Try to find macOS-specific asset or universal binary
        asset_name=$(echo "$release_info" | grep -o '"name": *"[^"]*\.tar\.gz"' | grep -i "macos\|darwin" | head -1 | sed 's/"name": *"//;s/"//')
        if [[ -z "$asset_name" ]]; then
            # Fallback to universal or x86_64 for macOS
            asset_name=$(echo "$release_info" | grep -o '"name": *"[^"]*\.tar\.gz"' | head -1 | sed 's/"name": *"//;s/"//')
        fi
    elif [[ "$platform" == "linux"* ]]; then
        asset_name=$(echo "$release_info" | grep -o '"name": *"[^"]*\.tar\.gz"' | grep -i "linux" | head -1 | sed 's/"name": *"//;s/"//')
    elif [[ "$platform" == "windows"* ]]; then
        asset_name=$(echo "$release_info" | grep -o '"name": *"[^"]*\.zip"' | grep -i "windows" | head -1 | sed 's/"name": *"//;s/"//')
    fi
    
    if [[ -z "$asset_name" ]]; then
        log_error "No compatible asset found for platform: $platform"
        log_info "Available assets:"
        echo "$release_info" | grep -o '"name": *"[^"]*\.\(tar\.gz\|zip\)"' | sed 's/"name": *"//;s/"//g' | sed 's/^/  - /'
        exit 1
    fi
    
    # Get download URL
    download_url=$(echo "$release_info" | grep -A 1 "\"name\": *\"$asset_name\"" | grep -o '"browser_download_url": *"[^"]*"' | sed 's/"browser_download_url": *"//;s/"//')
    
    if [[ -z "$download_url" ]]; then
        log_error "Could not find download URL for $asset_name"
        exit 1
    fi
    
    log_info "Found asset: $asset_name"
    log_info "Download URL: $download_url"
    
    # Check permissions
    if ! check_permissions; then
        log_info "Retrying with sudo..."
        exec sudo "$0" "$@"
    fi
    
    # Download and install
    download_and_install "$platform" "$latest_version" "$download_url" "$asset_name"
    
    # Verify installation
    log_info "Verifying installation..."
    if command -v $BINARY_NAME &> /dev/null; then
        local new_version=$($BINARY_NAME --version 2>/dev/null | grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' || echo "unknown")
        if [[ "$new_version" == "$latest_version" ]]; then
            log_success "Update completed successfully!"
            log_info "Updated from $current_version to $new_version"
        else
            log_warning "Installation completed but version mismatch detected"
            log_info "Expected: $latest_version, Got: $new_version"
        fi
    else
        log_error "Binary not found in PATH after installation"
        log_info "You may need to add $INSTALL_DIR to your PATH"
    fi
    
    # Cleanup
    rm -rf "$TEMP_DIR"
}

# Handle command line arguments
case "${1:-}" in
    --help|-h)
        echo "ROTD Manual Update Script"
        echo "Usage: $0 [options]"
        echo
        echo "Options:"
        echo "  --help, -h     Show this help message"
        echo "  --version, -v  Show current version and exit"
        echo "  --check, -c    Check for updates without installing"
        echo
        echo "Environment Variables:"
        echo "  INSTALL_DIR    Installation directory (default: /usr/local/bin)"
        echo
        exit 0
        ;;
    --version|-v)
        echo "Current ROTD version: $(get_current_version)"
        exit 0
        ;;
    --check|-c)
        current_version=$(get_current_version)
        release_info=$(curl -s "$GITHUB_API_URL")
        latest_version=$(echo "$release_info" | grep -o '"tag_name": *"[^"]*"' | grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+')
        
        echo "Current version: $current_version"
        echo "Latest version: $latest_version"
        
        if [[ "$current_version" == "$latest_version" ]]; then
            echo "Status: Up to date"
        else
            echo "Status: Update available"
        fi
        exit 0
        ;;
    "")
        # No arguments, proceed with update
        main
        ;;
    *)
        log_error "Unknown argument: $1"
        echo "Use --help for usage information"
        exit 1
        ;;
esac