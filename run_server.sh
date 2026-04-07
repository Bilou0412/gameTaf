#!/bin/bash
echo "╔════════════════════════════════════════╗"
echo "║  🎮 TERMIGAME PONG - Serveur  🎮      ║"
echo "╚════════════════════════════════════════╝"
echo ""
echo "Compilation et lancement du serveur..."
echo "Port utilisé: 9999"
echo ""
cargo run --release --bin server
