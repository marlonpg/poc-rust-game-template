# Rust Game Backend — Setup

This guide sets up a reliable Rust development environment for building a game backend. Choose the section for your OS.

## Windows Setup

This section sets up Rust on Windows.

### Prerequisites
- Windows 10/11 with `winget` available
- Visual Studio Code installed (with `code` CLI in PATH)

### Install Tooling

1) Install Rustup (toolchain manager):

```powershell
winget install --id Rustlang.Rustup -e --silent --accept-package-agreements --accept-source-agreements
```

2) Configure Rust (stable) and components:

```powershell
# Refresh PATH in the current PowerShell session
$env:PATH = [System.Environment]::GetEnvironmentVariable('PATH','Machine') + ';' + [System.Environment]::GetEnvironmentVariable('PATH','User')

rustup default stable
rustup component add rustfmt clippy
```

3) Install MSVC Build Tools (required for native linking):

```powershell
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --accept-package-agreements --accept-source-agreements --override "--quiet --wait --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

4) Install helpful VS Code extensions:

```powershell
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension serayuzgur.crates
```

### Verify Installation

```powershell
rustup -V
rustc -V
cargo -V
rustup show
```

Optional quick check:

```powershell
cd <your-workspace>
cargo new --bin _rust-check
cd _rust-check
cargo run
```

Expected output:

```
Hello, world!
```

### Next Steps
- Initialize your backend crate (e.g., `cargo new --bin server`)
- Add dependencies (e.g., `actix-web` or `axum`, `serde`, `tokio`)
- Set up formatting/linting: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`

### Troubleshooting
- If linking fails, ensure MSVC Build Tools are installed (step 3).
- If `code` CLI is not found, enable VS Code “Shell Command: Install 'code' command in PATH” or reinstall VS Code.
- If `winget` is missing, install it from Microsoft Store or use the official Rust installer: https://rustup.rs/

## Linux Setup

This section sets up Rust on Linux (Debian/Ubuntu, Fedora/RHEL/CentOS, Arch). Adjust package manager commands as needed.

### Prerequisites
- A Bash-compatible shell
- `curl` and `git` installed
- Visual Studio Code installed (optional, with `code` CLI)

Install common build tools:

```bash
# Debian/Ubuntu
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev curl git

# Fedora/RHEL/CentOS
sudo dnf groupinstall -y "Development Tools"
sudo dnf install -y pkg-config openssl-devel curl git

# Arch Linux
sudo pacman -S --needed base-devel openssl curl git
```

### Install Tooling

1) Install Rustup (toolchain manager):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

Load Rust environment for the current shell:

```bash
source "$HOME/.cargo/env"
```

2) Configure Rust (stable) and components:

```bash
rustup default stable
rustup component add rustfmt clippy
```

3) Install helpful VS Code extensions (optional):

```bash
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension serayuzgur.crates
```

### Verify Installation

```bash
rustup -V
rustc -V
cargo -V
rustup show
```

Optional quick check:

```bash
cd <your-workspace>
cargo new --bin _rust-check
cd _rust-check
cargo run
```

Expected output:

```
Hello, world!
```

### Next Steps
- Initialize your backend crate (e.g., `cargo new --bin server`)
- Add dependencies (e.g., `actix-web` or `axum`, `serde`, `tokio`)
- Set up formatting/linting: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`

### Troubleshooting
- If linker errors occur, ensure build tools are installed (e.g., `build-essential` on Debian/Ubuntu, `Development Tools` group on Fedora/RHEL).
- If `cargo` isn’t found in a new shell, add `source "$HOME/.cargo/env"` to your shell RC (e.g., `~/.bashrc` or `~/.zshrc`).
- For OpenSSL-dependent crates, ensure `libssl-dev` (Debian/Ubuntu) or `openssl-devel` (Fedora/RHEL) is installed.