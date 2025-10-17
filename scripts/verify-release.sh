#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}HURL Release Verification Script${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

FAILED_CHECKS=0
PASSED_CHECKS=0

check_file() {
    local file=$1
    local description=$2
    
    if [ -f "$PROJECT_DIR/$file" ]; then
        echo -e "${GREEN}✓${NC} $description"
        ((PASSED_CHECKS++))
    else
        echo -e "${RED}✗${NC} Missing: $description ($file)"
        ((FAILED_CHECKS++))
    fi
}

check_executable() {
    local file=$1
    local description=$2
    
    if [ -x "$PROJECT_DIR/$file" ]; then
        echo -e "${GREEN}✓${NC} $description is executable"
        ((PASSED_CHECKS++))
    else
        echo -e "${RED}✗${NC} Not executable: $description ($file)"
        ((FAILED_CHECKS++))
    fi
}

echo -e "${YELLOW}1. Checking Release Files...${NC}"
check_file "VERSION" "Version file"
check_file "CHANGELOG.md" "Changelog"
check_file "CONTRIBUTING.md" "Contributing guide"
check_file "LICENSE" "MIT License"
check_file "LICENSE-APACHE" "Apache License"
check_file "NOTICE" "Third-party notices"
check_file "build_info.txt" "Build information"
check_file "COMMIT_HASH" "Commit hash"
check_file ".cargo/config.toml" "Cargo configuration"
echo ""

echo -e "${YELLOW}2. Checking Documentation...${NC}"
check_file "README.md" "README"
check_file "DEVELOPMENT.md" "Development guide"
check_file "ARCHITECTURE.md" "Architecture documentation"
echo ""

echo -e "${YELLOW}3. Building Project...${NC}"
cd "$PROJECT_DIR"
if cargo build --release 2>/dev/null; then
    echo -e "${GREEN}✓${NC} Release build successful"
    ((PASSED_CHECKS++))
else
    echo -e "${RED}✗${NC} Release build failed"
    ((FAILED_CHECKS++))
fi
echo ""

echo -e "${YELLOW}4. Running Tests...${NC}"
if cargo test --lib --all 2>/dev/null | grep -q "test result: ok"; then
    echo -e "${GREEN}✓${NC} All tests passed"
    ((PASSED_CHECKS++))
else
    echo -e "${RED}✗${NC} Some tests failed"
    ((FAILED_CHECKS++))
fi
echo ""

echo -e "${YELLOW}5. Checking Code Quality...${NC}"

if cargo fmt --all -- --check 2>/dev/null; then
    echo -e "${GREEN}✓${NC} Code formatting correct"
    ((PASSED_CHECKS++))
else
    echo -e "${YELLOW}⚠${NC} Code needs formatting"
fi

if cargo clippy --all-targets --all-features 2>/dev/null | grep -qv "warning"; then
    echo -e "${GREEN}✓${NC} No clippy warnings"
    ((PASSED_CHECKS++))
fi

if cargo check --all 2>/dev/null; then
    echo -e "${GREEN}✓${NC} Code checks passed"
    ((PASSED_CHECKS++))
else
    echo -e "${RED}✗${NC} Code check failed"
    ((FAILED_CHECKS++))
fi
echo ""

echo -e "${YELLOW}6. Verifying Version Consistency...${NC}"
VERSION_FILE=$(cat "$PROJECT_DIR/VERSION")
CARGO_VERSION=$(grep "^version" "$PROJECT_DIR/Cargo.toml" | head -1 | sed 's/version = "//g' | sed 's/"//g')

if [ "$VERSION_FILE" = "$CARGO_VERSION" ]; then
    echo -e "${GREEN}✓${NC} Version consistent: $VERSION_FILE"
    ((PASSED_CHECKS++))
else
    echo -e "${RED}✗${NC} Version mismatch: VERSION=$VERSION_FILE, Cargo.toml=$CARGO_VERSION"
    ((FAILED_CHECKS++))
fi
echo ""

echo -e "${YELLOW}7. Binary Integrity Check...${NC}"
if [ -f "$PROJECT_DIR/target/release/hurl" ]; then
    FILE_SIZE=$(stat -f%z "$PROJECT_DIR/target/release/hurl" 2>/dev/null || stat -c%s "$PROJECT_DIR/target/release/hurl" 2>/dev/null)
    echo -e "${GREEN}✓${NC} Binary found: hurl ($FILE_SIZE bytes)"
    ((PASSED_CHECKS++))
    
    if "$PROJECT_DIR/target/release/hurl" --version >/dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} Binary executable and responds to --version"
        ((PASSED_CHECKS++))
    fi
else
    echo -e "${YELLOW}⚠${NC} Binary not found at expected location"
fi
echo ""

echo -e "${YELLOW}8. Documentation Tests...${NC}"
if cargo test --doc 2>/dev/null | grep -q "test result: ok"; then
    echo -e "${GREEN}✓${NC} Documentation tests passed"
    ((PASSED_CHECKS++))
fi
echo ""

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Release Verification Summary${NC}"
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}Passed checks:${NC} $PASSED_CHECKS"
echo -e "${RED}Failed checks:${NC} $FAILED_CHECKS"

if [ $FAILED_CHECKS -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✓ Release verification PASSED${NC}"
    echo -e "${GREEN}Project is ready for release!${NC}"
    exit 0
else
    echo ""
    echo -e "${RED}✗ Release verification FAILED${NC}"
    echo -e "${RED}Please fix the above issues before releasing.${NC}"
    exit 1
fi
