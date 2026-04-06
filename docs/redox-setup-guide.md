# Redox OS Setup Guide for X-OS

## Prerequisites

- Linux host (Ubuntu 22.04+ recommended) or macOS
- 8 GB RAM minimum, 16 GB recommended
- 30 GB free disk space
- Rust nightly toolchain
- QEMU for testing

## Step 1: Install Build Dependencies

```bash
# Ubuntu / Debian
sudo apt-get install -y \
    build-essential cmake curl fuse3 git libfuse3-dev \
    nasm pkg-config qemu-system-x86 qemu-utils \
    texinfo wget

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup default nightly
```

## Step 2: Clone Redox

```bash
# Clone the Redox project (includes all submodules)
mkdir -p ~/projects
cd ~/projects
git clone https://gitlab.redox-os.org/redox-os/redox.git --recursive
cd redox
```

## Step 3: First Build & Boot

```bash
# Bootstrap the build system (downloads toolchain, builds relibc, etc.)
# This takes 30-60 minutes on first run.
make all

# Boot in QEMU
make qemu
```

You should see the Redox desktop (Orbital) boot up. Default login: `user` / (no password).

## Step 4: Understanding Redox Architecture

Redox is a **microkernel**. Unlike Linux, most OS services run in userspace:

```
┌─────────────────────────────────────────┐
│              Applications               │
├─────────────────────────────────────────┤
│           Scheme Daemons                │
│  ┌──────┐ ┌──────┐ ┌────────┐          │
│  │netd  │ │filed │ │wallet: │ ← Xorion │
│  └──────┘ └──────┘ └────────┘          │
├─────────────────────────────────────────┤
│              relibc (libc)              │
├─────────────────────────────────────────┤
│     Kernel (memory, scheduling, IPC)    │
│     System calls: open/read/write/close │
└─────────────────────────────────────────┘
```

### Key Concepts

- **Schemes**: Everything is a URL. Files are `file:/path`, network is
  `tcp:/addr:port`, display is `display:`. Our wallet will be `wallet:`.

- **Scheme daemons**: Userspace processes that register a scheme name with
  the kernel and handle open/read/write/close requests from other processes.

- **No kernel networking**: The kernel has no TCP stack. Networking is handled
  by `netd` in userspace. This means wallet RPC calls *must* be in userspace.

### Where Schemes Live

```
redox/
├── kernel/          # Microkernel — scheduling, memory, IPC only
├── relibc/          # C library / Rust std implementation for Redox
├── schemes/         # Userspace scheme daemons ← Xorion goes here
│   ├── netd/        # Network daemon
│   ├── ramfs/       # RAM filesystem
│   └── ...
├── programs/        # Userspace programs
└── cookbook/         # Build recipes for all packages
```

## Step 5: Add Xorion as a Scheme

After the base system builds, we add Xorion as a new scheme daemon.
See the `xorion-scheme/` directory in this repository for the implementation.

## Step 6: Build Recipe

Create `cookbook/recipes/xorion-scheme/recipe.toml`:

```toml
[source]
git = "https://github.com/IDevNation/xorion-web3-os.git"
branch = "main"

[build]
template = "cargo"
```

Add to `config/desktop.toml` under `[packages]`:

```toml
xorion-scheme = {}
```

Rebuild:

```bash
make rebuild
make qemu
```
