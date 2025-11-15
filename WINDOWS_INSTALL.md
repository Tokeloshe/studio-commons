# Windows Installation Guide for Studio Commons

## Complete Step-by-Step Installation for Windows Users

This guide will help you install and run Studio Commons on Windows.

---

## Step 1: Install Microsoft Visual C++ Build Tools (REQUIRED!)

**Windows users MUST install this first or Rust won't compile!**

### Quick Install (Recommended):

Download and install **Visual Studio Build Tools 2022**:
1. Visit: https://visualstudio.microsoft.com/downloads/
2. Scroll down to "Tools for Visual Studio"
3. Download **"Build Tools for Visual Studio 2022"**
4. Run the installer
5. When prompted, select **"Desktop development with C++"**
6. Click Install (this will take 10-20 minutes)
7. Restart your computer after installation

**OR** use this direct download link:
https://aka.ms/vs/17/release/vs_BuildTools.exe

### Verify Installation:

After restarting, open PowerShell and run:
```powershell
where.exe link.exe
```

You should see a path like `C:\Program Files\Microsoft Visual Studio\...\link.exe`

---

## Step 2: Install Rust and Cargo

### Option A: Quick Install (Recommended)

1. **Download Rust Installer**
   - Visit: https://rustup.rs/
   - Click "Download rustup-init.exe (64-bit)"
   - Or use this direct link: https://win.rustup.rs/x86_64

2. **Run the Installer**
   - Double-click `rustup-init.exe`
   - Press `1` and Enter to proceed with default installation
   - Wait for installation to complete (may take 5-10 minutes)

3. **Restart PowerShell**
   - **IMPORTANT**: Close and reopen PowerShell for changes to take effect

### Option B: PowerShell Install

Run this in PowerShell (as Administrator):

```powershell
# Download and install Rust
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
Start-Process -FilePath "$env:TEMP\rustup-init.exe" -ArgumentList "-y" -Wait

# Add Rust to PATH (for current session)
$env:Path += ";$env:USERPROFILE\.cargo\bin"
```

**Then restart PowerShell!**

### Verify Rust Installation

After restarting PowerShell, verify the installation:

```powershell
cargo --version
rustc --version
```

You should see output like:
```
cargo 1.XX.X
rustc 1.XX.X
```

If you see "command not found", restart PowerShell again.

---

## Step 3: Install Git (if not already installed)

### Check if Git is installed:

```powershell
git --version
```

### If Git is not installed:

1. Download Git from: https://git-scm.com/download/win
2. Run the installer with default options
3. Restart PowerShell

---

## Step 4: Clone Studio Commons

**✅ CORRECT URL (Fixed!):**

```powershell
# Navigate to your preferred directory
cd $HOME\Documents

# Clone the repository
git clone https://github.com/Tokeloshe/studio-commons.git

# Enter the directory
cd studio-commons

# IMPORTANT: Checkout the development branch with all the code
git checkout claude/fix-asp-validation-0156uG4x62SsimRfcQbtWZz2
```

**⚠️ IMPORTANT**: The complete v1.0.0 code is on the `claude/fix-asp-validation-0156uG4x62SsimRfcQbtWZz2` branch. You MUST run `git checkout claude/fix-asp-validation-0156uG4x62SsimRfcQbtWZz2` after cloning or you'll get "Cargo.toml not found" errors!

---

## Step 5: Build the Project

```powershell
# Build in release mode (optimized)
cargo build --release
```

This will take 5-15 minutes the first time as it downloads and compiles dependencies.

**Expected output:** You'll see many "Compiling..." messages, ending with:
```
Finished `release` profile [optimized] target(s) in XX.XXs
```

---

## Step 6: Run Tests

Verify everything works:

```powershell
cargo test --all
```

**Expected output:** All tests should pass:
```
test result: ok. XX passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## Step 7: Run Studio Commons

```powershell
# Run the application
cargo run --release
```

**Expected output:**

```
╔═══════════════════════════════════════════════════════════════╗
║          STUDIO COMMONS - Global Creative Infrastructure      ║
╠═══════════════════════════════════════════════════════════════╣
║  Version: 1.0.0                                               ║
║  License: AGPL-3.0                                            ║
║  Repository: github.com/Tokeloshe/studio-commons              ║
╠═══════════════════════════════════════════════════════════════╣
║  Founder's Fee: 1% to XRP wallet                              ║
║  Address: rf82s1CDagppvM6ATqc1nSrL6GackzHJrm                  ║
║  Memo: 2621443948                                             ║
╚═══════════════════════════════════════════════════════════════╝

Studio Commons initialized successfully!
Region: LA

Available commands:
  - global-book: Book resources across hubs
  - vote: Participate in governance
  - contribute: Track creative contributions
  - analytics: View impact predictions
```

---

## All-in-One Installation Script

Save this as `install-studio-commons.ps1` and run it:

```powershell
# Studio Commons - Windows Installation Script
# Run this in PowerShell (as Administrator recommended)

Write-Host "`n╔════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║     STUDIO COMMONS - Windows Installation Script      ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

# Step 1: Check for MSVC Build Tools
Write-Host "[1/6] Checking for Microsoft Visual C++ Build Tools..." -ForegroundColor Yellow
if (!(Get-Command link.exe -ErrorAction SilentlyContinue)) {
    Write-Host "      ✗ MSVC Build Tools not found!" -ForegroundColor Red
    Write-Host "" -ForegroundColor White
    Write-Host "      You MUST install Visual Studio Build Tools first:" -ForegroundColor Yellow
    Write-Host "      1. Download from: https://aka.ms/vs/17/release/vs_BuildTools.exe" -ForegroundColor White
    Write-Host "      2. Run the installer" -ForegroundColor White
    Write-Host "      3. Select 'Desktop development with C++'" -ForegroundColor White
    Write-Host "      4. Install (takes 10-20 minutes)" -ForegroundColor White
    Write-Host "      5. Restart your computer" -ForegroundColor White
    Write-Host "      6. Run this script again" -ForegroundColor White
    Write-Host "" -ForegroundColor White
    Write-Host "      Without this, Rust cannot compile on Windows!" -ForegroundColor Red
    exit
} else {
    Write-Host "      ✓ MSVC Build Tools found" -ForegroundColor Green
}

# Step 2: Check/Install Rust
Write-Host "[2/6] Checking Rust installation..." -ForegroundColor Yellow
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "      Installing Rust... (this may take 10 minutes)" -ForegroundColor Yellow
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
    Start-Process -FilePath "$env:TEMP\rustup-init.exe" -ArgumentList "-y" -Wait

    # Add to PATH for current session
    $env:Path += ";$env:USERPROFILE\.cargo\bin"

    Write-Host "      ✓ Rust installed!" -ForegroundColor Green
    Write-Host "      Please restart PowerShell and run this script again.`n" -ForegroundColor Cyan
    exit
} else {
    Write-Host "      ✓ Rust is already installed" -ForegroundColor Green
}

# Step 3: Check Git
Write-Host "[3/6] Checking Git installation..." -ForegroundColor Yellow
if (!(Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Host "      ✗ Git not found. Please install from: https://git-scm.com/download/win" -ForegroundColor Red
    exit
} else {
    Write-Host "      ✓ Git is installed" -ForegroundColor Green
}

# Step 4: Clone Repository
Write-Host "[4/6] Cloning Studio Commons..." -ForegroundColor Yellow
$targetPath = "$HOME\Documents\studio-commons"
if (Test-Path $targetPath) {
    Write-Host "      Directory exists. Updating..." -ForegroundColor Yellow
    cd $targetPath
    git fetch
    git checkout claude/fix-asp-validation-0156uG4x62SsimRfcQbtWZz2
    git pull
} else {
    cd $HOME\Documents
    git clone https://github.com/Tokeloshe/studio-commons.git
    cd studio-commons
    # CRITICAL: Checkout the branch with all the code
    git checkout claude/fix-asp-validation-0156uG4x62SsimRfcQbtWZz2
}
Write-Host "      ✓ Repository ready" -ForegroundColor Green

# Step 5: Build Project
Write-Host "[5/6] Building Studio Commons... (first build takes 5-15 min)" -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -eq 0) {
    Write-Host "      ✓ Build successful!" -ForegroundColor Green
} else {
    Write-Host "      ✗ Build failed" -ForegroundColor Red
    exit
}

# Step 6: Run Tests
Write-Host "[6/6] Running tests..." -ForegroundColor Yellow
cargo test --all --quiet
if ($LASTEXITCODE -eq 0) {
    Write-Host "      ✓ All tests passed!" -ForegroundColor Green
} else {
    Write-Host "      ⚠ Some tests failed (may still work)" -ForegroundColor Yellow
}

# Success!
Write-Host "`n╔════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║              INSTALLATION SUCCESSFUL! ✓                ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════════════════════╝`n" -ForegroundColor Green

Write-Host "Studio Commons v1.0.0 is ready to use!`n" -ForegroundColor Cyan

Write-Host "To run the platform:" -ForegroundColor White
Write-Host "  cargo run --release`n" -ForegroundColor Cyan

Write-Host "XRP Founder Fee Configuration:" -ForegroundColor White
Write-Host "  Wallet: rf82s1CDagppvM6ATqc1nSrL6GackzHJrm" -ForegroundColor Cyan
Write-Host "  Memo: 2621443948" -ForegroundColor Cyan
Write-Host "  Fee: 1% of net profits`n" -ForegroundColor Cyan

Write-Host "Location: $targetPath`n" -ForegroundColor Gray

Write-Host "Documentation:" -ForegroundColor White
Write-Host "  README.md - Full documentation" -ForegroundColor Gray
Write-Host "  CONTRIBUTING.md - Contribution guide`n" -ForegroundColor Gray

Write-Host "For help: https://github.com/Tokeloshe/studio-commons/issues`n" -ForegroundColor White
```

To run this script:

```powershell
# Save the script to a file, then run:
.\install-studio-commons.ps1
```

---

## Troubleshooting

### ❌ ERROR: "linker `link.exe` not found" (MOST COMMON!)

**Full error message:**
```
error: linker `link.exe` not found
note: the msvc targets depend on the msvc linker but `link.exe` was not found
note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio were installed with the Visual C++ option.
```

**Problem**: You don't have Microsoft Visual C++ Build Tools installed!

**Solution**:
1. Download Visual Studio Build Tools: https://aka.ms/vs/17/release/vs_BuildTools.exe
2. Run the installer
3. When prompted, check **"Desktop development with C++"**
4. Click Install (takes 10-20 minutes)
5. **Restart your computer** (very important!)
6. Try building again: `cargo build --release`

**This is the #1 issue for Windows users!** Rust on Windows REQUIRES MSVC Build Tools to compile code.

**Verify it's installed:**
```powershell
where.exe link.exe
```
You should see: `C:\Program Files\Microsoft Visual Studio\...\link.exe`

---

### "could not find Cargo.toml" error

**Problem**: You cloned the repository but didn't checkout the branch with the code!

**Solution**: Run this command in the studio-commons directory:
```powershell
git checkout claude/fix-asp-validation-0156uG4x62SsimRfcQbtWZz2
```

Then try building again:
```powershell
cargo build --release
```

The complete v1.0.0 code is on the `claude/fix-asp-validation-0156uG4x62SsimRfcQbtWZz2` branch, not on `main`.

### "cargo: command not found" after installing Rust

**Solution**: Close PowerShell completely and open a new window.

If still not working:
```powershell
# Manually add Rust to PATH
$env:Path += ";$env:USERPROFILE\.cargo\bin"
```

### "Repository not found"

Make sure you're using the **correct URL**:
```powershell
git clone https://github.com/Tokeloshe/studio-commons.git
```

**NOT** `e_honiball/studio-commons` (old URL)

### Build errors

```powershell
# Update Rust to latest version
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Slow internet/downloads

The first build downloads ~100MB of dependencies. On slow connections:
- Be patient (may take 30+ minutes)
- Make sure you're on a stable connection
- Consider building with: `cargo build` (debug mode, faster but unoptimized)

### Permission errors

Run PowerShell as Administrator:
1. Right-click PowerShell icon
2. Select "Run as Administrator"

---

## Quick Reference

### Common Commands

```powershell
# Build the project
cargo build --release

# Run tests
cargo test --all

# Run the application
cargo run --release

# Run for a specific region
$env:STUDIO_REGION="MUMBAI"
cargo run --release

# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for issues
cargo check
```

### Project Structure

```
studio-commons/
├── Cargo.toml           # Project configuration
├── README.md            # Main documentation
├── CONTRIBUTING.md      # Contribution guide
├── LICENSE              # AGPL-3.0 license
└── src/
    ├── main.rs          # Application entry
    ├── governance/      # DAO & voting
    ├── treasury/        # DeFi integration
    ├── payments/        # XRP fee automation
    ├── cci/             # Contribution tracking
    ├── production/      # AI tools
    ├── membership/      # Member management
    ├── analytics/       # Predictions
    ├── compliance/      # Legal adapters
    └── utils/           # Shared utilities
```

---

## Next Steps

1. ✅ Install Rust
2. ✅ Clone repository
3. ✅ Build and test
4. ✅ Run Studio Commons
5. 📖 Read [README.md](README.md) for full features
6. 🤝 Read [CONTRIBUTING.md](CONTRIBUTING.md) to contribute
7. 🌍 Help build the global creative commons!

---

## Support

- **Repository**: https://github.com/Tokeloshe/studio-commons
- **Issues**: https://github.com/Tokeloshe/studio-commons/issues
- **Contact**: [@e_honiball](https://x.com/e_honiball) on X

---

**Welcome to Studio Commons - Building the future of community-owned creative infrastructure!**

*XRP Founder Fee: 1% to wallet rf82s1CDagppvM6ATqc1nSrL6GackzHJrm (memo: 2621443948)*
