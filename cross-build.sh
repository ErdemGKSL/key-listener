#!/bin/bash

# Cross-compilation build script for key-listener
# Builds for multiple targets

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Available targets
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-pc-windows-gnu"
    "i686-unknown-linux-gnu"
    "i686-pc-windows-gnu"
)

echo -e "${GREEN}Cross-compilation build script for key-listener${NC}"
echo ""

# Function to build for a specific target
build_target() {
    local target=$1
    local feature=$2
    local bin_name=$3
    
    echo -e "${YELLOW}Building $bin_name for $target...${NC}"
    
    if [ "$feature" = "all" ]; then
        cargo build --release --target "$target"
    else
        cargo build --release --target "$target" --bin "$bin_name" --features "$feature" --no-default-features
    fi
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Successfully built $bin_name for $target${NC}"
    else
        echo -e "${RED}✗ Failed to build $bin_name for $target${NC}"
    fi
    echo ""
}

# Check if specific target is requested
if [ $# -eq 1 ]; then
    TARGET=$1
    echo "Building for specific target: $TARGET"
    
    # Install target if not already installed
    rustup target add "$TARGET"
    
    # Build all variants for the target
    build_target "$TARGET" "all" "key-listener"
    build_target "$TARGET" "direct" "key-listener-direct"
    build_target "$TARGET" "complex" "key-listener-complex"
    build_target "$TARGET" "hold_and_release" "key-listener-hold-release"
    build_target "$TARGET" "simulation" "key-listener-simulation"
    build_target "$TARGET" "mouse" "key-listener-mouse"
    
    echo -e "${GREEN}Cross-compilation completed for $TARGET${NC}"
    echo "Binaries are located in: target/$TARGET/release/"
    exit 0
fi

# Build for all targets
echo "Building for all supported targets..."
echo ""

for target in "${TARGETS[@]}"; do
    echo -e "${YELLOW}Installing target $target...${NC}"
    rustup target add "$target"
    
    if [ $? -eq 0 ]; then
        build_target "$target" "all" "key-listener"
        build_target "$target" "direct" "key-listener-direct"
        build_target "$target" "complex" "key-listener-complex"
        build_target "$target" "hold_and_release" "key-listener-hold-release"
        build_target "$target" "simulation" "key-listener-simulation"
        build_target "$target" "mouse" "key-listener-mouse"
    else
        echo -e "${RED}✗ Failed to install target $target${NC}"
    fi
    echo ""
done

echo -e "${GREEN}Cross-compilation completed for all targets${NC}"
echo ""
echo "Usage: ./cross-build.sh [target]"
echo "  Without target: builds for all supported targets"
echo "  With target: builds only for specified target"
echo ""
echo "Supported targets:"
for target in "${TARGETS[@]}"; do
    echo "  - $target"
done
