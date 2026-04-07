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
        mkdir -p "$INSTALL_DIR"
        cd "$INSTALL_DIR"
        
        if command -v curl &> /dev/null; then
            curl -fsSL "https://github.com/Bilou0412/gameTaf/archive/main.zip" -o /tmp/gametaf.zip
            unzip -q /tmp/gametaf.zip -d /tmp/
            mv /tmp/gameTaf-main/* "$INSTALL_DIR/" 2>/dev/null || mv /tmp/gameTaf-main/.* "$INSTALL_DIR/" 2>/dev/null || true
            rm -rf /tmp/gametaf.zip /tmp/gameTaf-main
        else
            echo "[ERROR] curl or git required"
            exit 1
        fi
    }
fi

cd "$INSTALL_DIR"

# Build the project
echo "[*] Building project..."
if ! cargo build --release 2>/dev/null; then
    echo "[ERROR] Build failed. Make sure Rust is installed (https://rustup.rs)"
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

show_menu() {
    clear
    echo "======================================="
    echo "  GT - Main Menu"
    echo "======================================="
    echo ""
    echo "  1 - Start Server"
    echo "  2 - Join Game (Client)"
    echo "  3 - Show Controls"
    echo "  4 - Exit"
    echo ""
    echo -n "Choose option (1-4): "
}

show_controls() {
    clear
    echo "======================================="
    echo "  GT - Game Controls"
    echo "======================================="
    echo ""
    echo "  W or Z  - Move paddle up"
    echo "  S      - Move paddle down"
    echo "  Q      - Quit game"
    echo ""
    echo "  Ball bounces on surfaces"
    echo "  First to 3 points wins!"
    echo ""
    read -p "Press Enter to return to menu..."
}

run_server() {
    clear
    echo "Starting Server..."
    echo ""
    
    if [ -f "$BIN_DIR/gt-server" ]; then
        exec "$BIN_DIR/gt-server"
    else
        echo "Building server..."
        cd "$INSTALL_DIR" && cargo run --release --bin server
    fi
}

run_client() {
    clear
    echo "Starting Client..."
    echo ""
    
    if [ -f "$BIN_DIR/gt-client" ]; then
        exec "$BIN_DIR/gt-client"
    else
        echo "Building client..."
        cd "$INSTALL_DIR" && cargo run --release --bin client
    fi
}

while true; do
    show_menu
    read -r choice
    
    case $choice in
        1)
            run_server
            ;;
        2)
            run_client
            ;;
        3)
            show_controls
            ;;
        4)
            echo "Goodbye"
            exit 0
            ;;
        *)
            echo "Invalid option. Press Enter to retry..."
            read
            ;;
    esac
done
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
echo "======================================="
echo "  Installation Complete!"
echo "======================================="
echo ""
echo "Quick start:"
echo "  gt"
echo ""
echo "Installation directory:"
echo "  $INSTALL_DIR"
echo ""
echo "Reload shell:"
echo "  source $SHELL_RC"
echo ""
echo "Or open a new terminal."
echo ""
