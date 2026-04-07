# GT - Installation Guide

One-line simple and fast installation!

## Quick Install (One-liner)

```bash
curl -fsSL https://raw.githubusercontent.com/Bilou0412/gameTaf/main/install.sh | bash
```

---

## After Installation

Once installed, simply run:

```bash
gt
```

This launches an interactive menu where you can:
- Start Server
- Join Game (Client)
- Show Controls

---

## Requirements

### Required
- **Git** (for cloning) - Or `curl` as fallback
- **Rust 1.70+** (optional if you have pre-compiled binaries)

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Game Controls

| Key | Action |
|-----|--------|
| **W / Z** | Move paddle up |
| **S** | Move paddle down |
| **Q** | Quit |

---

## Playing the Game

### Method 1: Same Computer (localhost)
```bash
# Terminal 1 - Server
gt
# Choose option 1

# Terminal 2 - Client 1
gt
# Choose option 2
# Enter: 127.0.0.1

# Terminal 3 - Client 2
gt
# Choose option 2
# Enter: 127.0.0.1
```

### Method 2: Over Network (LAN)
```bash
# On the server computer
gt
# Choose option 1
# Note the IP address shown (e.g., 192.168.1.100)

# On client computers
gt
# Choose option 2
# Enter the server IP address
```

---

## Uninstall

```bash
rm -rf ~/.gt
```

Then remove the `GT installation` line from your `~/.zshrc` or `~/.bashrc`.

---

## Update

Simply run the install script again:
```bash
curl -fsSL https://raw.githubusercontent.com/Bilou0412/gameTaf/main/install.sh | bash
```

---

## Troubleshooting

**"gt: command not found"**
```bash
source ~/.zshrc
# or
source ~/.bashrc
```

**"Build failed"**
Make sure Rust 1.70+ is installed:
```bash
rustup update
```

---

## About

Classic Pong game reimagined in the terminal with UDP networking!

[View on GitHub](https://github.com/Bilou0412/gameTaf)
