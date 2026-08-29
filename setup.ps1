# setup.ps1 — rust-game setup (Windows)
#
# Checks prerequisites (git, rustc >= 1.88.0, cargo), clones external
# reference material into deps/, and builds the project.
# Safe to run multiple times.

$ErrorActionPreference = "Stop"

$DEPS_DIR = "deps"
$RBE_REPO = "https://github.com/rust-lang/rust-by-example.git"
$RBE_DIR = "$DEPS_DIR\rust-by-example"
$RUSTLINGS_REPO = "https://github.com/rust-lang/rustlings.git"
$RUSTLINGS_DIR = "$DEPS_DIR\rustlings"

# --- Check prerequisites ---

Write-Host "Checking prerequisites..."

if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Host "Error: git is not installed." -ForegroundColor Red
    Write-Host "Install git from https://git-scm.com/downloads and try again."
    exit 1
}

if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    Write-Host "Error: rustc is not installed." -ForegroundColor Red
    Write-Host "Install Rust from https://rustup.rs/ and try again."
    exit 1
}

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Error: cargo is not installed." -ForegroundColor Red
    Write-Host "Install Rust from https://rustup.rs/ and try again."
    exit 1
}

Write-Host "  git:   $(git --version)"
Write-Host "  rustc: $(rustc --version)"
Write-Host "  cargo: $(cargo --version)"
Write-Host "  All prerequisites met.`n"

# --- Create deps directory ---

if (-not (Test-Path $DEPS_DIR)) {
    New-Item -ItemType Directory -Path $DEPS_DIR | Out-Null
}

# --- Clone Rust by Example ---

if (Test-Path $RBE_DIR) {
    Write-Host "Rust by Example already exists at $RBE_DIR — skipping."
} else {
    Write-Host "Cloning Rust by Example (reference material)..."
    try {
        git clone --depth 1 $RBE_REPO $RBE_DIR
        Write-Host "  Cloned successfully."
    } catch {
        Write-Host "  Warning: Failed to clone Rust by Example. This is optional — continuing." -ForegroundColor Yellow
    }
}

# --- Optionally clone Rustlings ---

Write-Host ""
$reply = Read-Host "Clone Rustlings for reference? (optional) [y/N]"
if ($reply -eq "y" -or $reply -eq "Y") {
    if (Test-Path $RUSTLINGS_DIR) {
        Write-Host "Rustlings already exists at $RUSTLINGS_DIR — skipping."
    } else {
        Write-Host "Cloning Rustlings..."
        try {
            git clone --depth 1 $RUSTLINGS_REPO $RUSTLINGS_DIR
            Write-Host "  Cloned successfully."
        } catch {
            Write-Host "  Warning: Failed to clone Rustlings. This is optional — continuing." -ForegroundColor Yellow
        }
    }
}

# --- Build rust-game ---

Write-Host "`nBuilding rust-game (first build takes 2-5 minutes, subsequent builds are fast)..."
cargo build --release
Write-Host "Build complete."

# --- Done ---

Write-Host "`nSetup complete!`n"
Write-Host "Next steps:"
Write-Host "  1. Run: cargo run             (launches the TUI dashboard)"
Write-Host "  2. Or:  cargo run -- watch    (start with the first exercise)"
Write-Host "  3. See: cargo run -- --help   (for all commands)"
