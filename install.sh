#!/bin/bash

# GT - Multiplayer Pong Game Installation Script

set -e

INSTALL_DIR="$HOME/.gt"
BIN_DIR="$INSTALL_DIR/bin"
APP_NAME="GT"

echo "======================================"
echo "  $APP_NAME - Installation"
echo "======================================"
echo ""

# Create installation directory
echo "[*] Creating installation directory..."
mkdir -p "$BIN_DIR"

# Clone or update repository
if [ -d "$INSTALL_DIR/.git" ]; then
    echo "[*] Updating existing installation..."
    cd "$INSTALL_DIR"
    git pull origin main 2>/dev/null || true
else
    echo "[*] Downloading project..."
    git clone https://github.com/Bilou0412/gameTaf.git "$INSTALL_DIR" 2>/dev/null || {
        echo "[*] Git not available, using direct download..."
        
        if command -v curl &> /dev/null; then
            TEMP_DIR="/tmp/gametaf-$$"
            mkdir -p "$TEMP_DIR"
            curl -fsSL "https://github.com/Bilou0412/gameTaf/archive/main.zip" -o "$TEMP_DIR/gametaf.zip"
            unzip -q -o "$TEMP_DIR/gametaf.zip" -d "$TEMP_DIR/"
            mkdir -p "$INSTALL_DIR"
            cp -r "$TEMP_DIR/gameTaf-main/"* "$INSTALL_DIR/" 2>/dev/null || true
            cp -r "$TEMP_DIR/gameTaf-main/".* "$INSTALL_DIR/" 2>/dev/null || true
            rm -rf "$TEMP_DIR"
        else
            echo "[ERROR] curl or git required"
            exit 1
        fi
    }
fi

cd "$INSTALL_DIR"

# Check Rust/cargo
if ! command -v cargo &>/dev/null; then
    echo "[ERROR] cargo not found. Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
echo "[*] Rust/cargo: $(cargo --version)"

# Build the project
echo "[*] Building project (this may take a few minutes)..."
if ! cargo build --release; then
    echo ""
    echo "[ERROR] Build failed (see errors above)"
    exit 1
fi

echo "[*] Installing binaries..."
cp target/release/client "$BIN_DIR/gt-client" 2>/dev/null || true
cp target/release/server "$BIN_DIR/gt-server" 2>/dev/null || true

# Create main launcher script
cat > "$BIN_DIR/gt" << 'LAUNCHER_EOF'
#!/bin/bash
INSTALL_DIR="$HOME/.gt"
BIN_DIR="$INSTALL_DIR/bin"

case "${1:-help}" in
    server)
        if [ -f "$BIN_DIR/gt-server" ]; then
            exec "$BIN_DIR/gt-server"
        else
            cd "$INSTALL_DIR" && exec cargo run --release --bin server 2>/dev/null
        fi
        ;;
    client)
        if [ -f "$BIN_DIR/gt-client" ]; then
            exec "$BIN_DIR/gt-client"
        else
            cd "$INSTALL_DIR" && exec cargo run --release --bin client 2>/dev/null
        fi
        ;;
    help|--help|-h|"")
        echo "GT - Quiz"
        echo "Usage: gt server|client"
        ;;
    *)
        echo "Unknown: $1" >&2
        exit 1
        ;;
esac
LAUNCHER_EOF

chmod +x "$BIN_DIR/gt"

# Add to PATH
SHELL_RC=""
if [ -f "$HOME/.zshrc" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -f "$HOME/.bashrc" ]; then
    SHELL_RC="$HOME/.bashrc"
else
    SHELL_RC="$HOME/.bash_profile"
fi

if ! grep -q "GT installation" "$SHELL_RC" 2>/dev/null; then
    echo "" >> "$SHELL_RC"
    echo "# GT installation" >> "$SHELL_RC"
    echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$SHELL_RC"
fi

# Success message
echo ""
echo "[*] Installation Complete!"
echo "[*] Commands:"
echo "  gt server - Start server"
echo "  gt client - Join game"
echo ""
