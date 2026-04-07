#!/bin/bash

# 🎮 TERMIGAME PONG - Installer
# Installation simple du jeu Pong multijoueur

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Install directory
INSTALL_DIR="$HOME/.termigame-pong"
BIN_DIR="$INSTALL_DIR/bin"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         🎮 TERMIGAME PONG - Installation 🎮                   ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if Rust is installed (optional, but recommended)
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}⚠️  Rust n'est pas installé.${NC}"
    echo -e "Installation disponible sur ${BLUE}https://rustup.rs/${NC}"
    echo ""
    read -p "Continuer sans Rust ? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo -e "${RED}Installation annulée.${NC}"
        exit 1
    fi
    NEED_COMPILE=true
else
    NEED_COMPILE=false
fi

# Create install directory
echo -e "${YELLOW}📁 Création du répertoire d'installation...${NC}"
mkdir -p "$BIN_DIR"

# Clone or update the repository
if [ -d "$INSTALL_DIR/.git" ]; then
    echo -e "${YELLOW}📦 Mise à jour du dossier existant...${NC}"
    cd "$INSTALL_DIR"
    git pull origin main 2>/dev/null || true
else
    echo -e "${YELLOW}📥 Téléchargement du projet...${NC}"
    git clone https://github.com/bilou0412/termigame-pong.git "$INSTALL_DIR" 2>/dev/null || {
        # Fallback if git is not available
        echo -e "${YELLOW}Git non disponible, téléchargement direct...${NC}"
        mkdir -p "$INSTALL_DIR"
        cd "$INSTALL_DIR"
        
        # Download with curl (simple fallback)
        if command -v curl &> /dev/null; then
            curl -fsSL "https://github.com/bilou0412/termigame-pong/archive/main.zip" -o /tmp/termigame.zip
            unzip -q /tmp/termigame.zip -d /tmp/
            mv /tmp/termigame-pong-main/* "$INSTALL_DIR/"
            rm -rf /tmp/termigame.zip /tmp/termigame-pong-main
        else
            echo -e "${RED}❌ Erreur: curl ou git requis${NC}"
            exit 1
        fi
    }
fi

cd "$INSTALL_DIR"

# Compile if Rust is available
if [ "$NEED_COMPILE" = false ]; then
    echo -e "${YELLOW}🔨 Compilation du projet...${NC}"
    cargo build --release 2>/dev/null || {
        echo -e "${RED}❌ Compilation échouée${NC}"
        exit 1
    }
    
    # Copy binaries
    echo -e "${YELLOW}📋 Installation des binaires...${NC}"
    cp target/release/client "$BIN_DIR/termigame-client" 2>/dev/null || true
    cp target/release/server "$BIN_DIR/termigame-server" 2>/dev/null || true
fi

# Create launcher script
cat > "$BIN_DIR/termigame-pong" << 'LAUNCHER_EOF'
#!/bin/bash

INSTALL_DIR="$HOME/.termigame-pong"
BIN_DIR="$INSTALL_DIR/bin"

# Colors
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

show_menu() {
    clear
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║         🎮 TERMIGAME PONG - Menu Principal 🎮                  ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "  ${GREEN}1${NC} - 🎮 Lancer le serveur"
    echo -e "  ${GREEN}2${NC} - 👾 Rejoindre une partie (client)"
    echo -e "  ${GREEN}3${NC} - 📖 Afficher les contrôles"
    echo -e "  ${GREEN}4${NC} - ❌ Quitter"
    echo ""
    echo -e "${YELLOW}Choisissez une option (1-4):${NC} "
}

show_controls() {
    clear
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║              🎮 Contrôles du Jeu 🎮                            ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "  ${GREEN}W ou Z${NC}  - Déplacer la raquette vers le haut"
    echo -e "  ${GREEN}S${NC}      - Déplacer la raquette vers le bas"
    echo -e "  ${GREEN}Q${NC}      - Quitter le jeu"
    echo ""
    echo "  🏐 Le ballon rebondit sur les surfaces!"
    echo "  🎯 Gagnez 3 points pour remporter la partie!"
    echo ""
    read -p "Appuyez sur Entrée pour revenir au menu..."
}

run_server() {
    clear
    echo -e "${GREEN}🎮 Lancement du serveur...${NC}"
    echo ""
    
    if [ -f "$BIN_DIR/termigame-server" ]; then
        exec "$BIN_DIR/termigame-server"
    else
        echo -e "${YELLOW}Compilation du serveur...${NC}"
        cd "$INSTALL_DIR" && cargo run --release --bin server
    fi
}

run_client() {
    clear
    echo -e "${GREEN}👾 Lancement du client...${NC}"
    echo ""
    
    if [ -f "$BIN_DIR/termigame-client" ]; then
        exec "$BIN_DIR/termigame-client"
    else
        echo -e "${YELLOW}Compilation du client...${NC}"
        cd "$INSTALL_DIR" && cargo run --release --bin client
    fi
}

# Main loop
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
            echo -e "${GREEN}Au revoir! 👋${NC}"
            exit 0
            ;;
        *)
            echo -e "${YELLOW}Option invalide. Appuyez sur Entrée pour réessayer...${NC}"
            read
            ;;
    esac
done
LAUNCHER_EOF

chmod +x "$BIN_DIR/termigame-pong"

# Add to PATH
SHELL_RC=""
if [ -f "$HOME/.zshrc" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -f "$HOME/.bashrc" ]; then
    SHELL_RC="$HOME/.bashrc"
else
    SHELL_RC="$HOME/.bash_profile"
fi

if ! grep -q "termigame-pong" "$SHELL_RC"; then
    echo "" >> "$SHELL_RC"
    echo "# TERMIGAME PONG PATH" >> "$SHELL_RC"
    echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$SHELL_RC"
fi

# Final message
echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║        ✅ Installation terminée avec succès! ✅                ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}🚀 Démarrage rapide:${NC}"
echo ""
echo -e "  ${BLUE}termigame-pong${NC}  - Lance le menu principal"
echo ""
echo -e "${YELLOW}⚙️  Répertoire d'installation:${NC}"
echo "  $INSTALL_DIR"
echo ""
echo -e "${YELLOW}📝 Recharger votre shell:${NC}"
echo "  ${BLUE}source $SHELL_RC${NC}"
echo ""
echo -e "${YELLOW}ou ouvrez un nouveau terminal.${NC}"
echo ""
