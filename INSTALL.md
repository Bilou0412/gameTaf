# GT - Installation Guide

One-line simple and fast installation!

## Quick Install (One-liner)

```bash
curl -fsSL https://raw.githubusercontent.com/Bilou0412/gameTaf/main/install.sh | bash
```

---

## After Installation

Start the game server:

```bash
gt server
```

Join the game from another terminal/computer:

```bash
gt client
```

Then enter the server IP address when prompted.

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
gt server

# Terminal 2 - Client
gt client
# When prompted: Enter 127.0.0.1
```

### Method 2: Over Network (LAN)
```bash
# Terminal 1 - On server computer
gt server

# Terminal 2+ - On client computers
gt client
# When prompted: Enter the server IP address
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

Trivia quiz game reimagined in the terminal with UDP networking!

[View on GitHub](https://github.com/Bilou0412/gameTaf)
