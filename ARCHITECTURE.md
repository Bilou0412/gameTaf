# 📋 Structure du Projet TermiGame Pong

## 📁 Arborescence

```
termigame/
├── Cargo.toml                 # Configuration Rust (dépendances)
├── Cargo.lock                 # Verrouillage des versions
├── README.md                  # Documentation complète
├── DEMARRAGE_RAPIDE.txt       # Guide d'installation
│
├── run_server.bat             # Script lancement serveur (Windows)
├── run_client.bat             # Script lancement client (Windows)
├── run_server.sh              # Script lancement serveur (Unix)
├── run_client.sh              # Script lancement client (Unix)
│
├── src/
│   ├── lib.rs                 # Exports des modules
│   ├── game.rs                # Logique du jeu (Pong)
│   ├── renderer.rs            # Affichage terminal ASCII
│   ├── input.rs               # Gestion des entrées clavier
│   │
│   └── bin/
│       ├── server.rs          # Serveur UDP (gère la partie)
│       └── client.rs          # Client UDP (interface joueur)
│
└── target/                    # Dossier de build (généré)
```

## 🎯 Architecture du Jeu

### **Communication Réseau**
- **Protocole**: UDP pour la basse latence
- **Port**: 9999
- **Format**: Messages sérialisés (bincode)
- **Modèle**: Client/Serveur

### **Messages Échangés**
```
PaddleMove(y)           → Client → Serveur (position raquette)
StateUpdate(GameState)  → Serveur → Clients (état du jeu)
Reset                   → Demande réinitialisation
```

### **Entités du Jeu**
- **Balle**: Position (x, y) + vélocité (vx, vy)
- **Raquettes**: 2 joueurs (gauche/droite), hauteur 3 caractères
- **Terrain**: 80×24 caractères + bordures
- **Score**: Remis à 0 quand la balle franchit les limites

## 🔧 Technologie

- **Langage**: Rust (2021 edition)
- **Dépendances minimales**:
  - `serde` - Sérialisation structurée
  - `bincode` - Sérialisation binaire compacte
- **Performance**: ~60 FPS (16ms par frame)

## 🎮 Flux de Jeu

1. **Serveur démarre** → Écoute sur UDP:9999
2. **Joueur 1 se connecte** → Spécifie adresse serveur
3. **Joueur 2 se connecte** → La partie commence
4. **Boucle de jeu** (serveur):
   - Reçoit mouvements des clients
   - Met à jour physique de la balle
   - Détecte collisions & points
   - Rédiffuse l'état aux clients
5. **Affichage (clients)**:
   - Reçoit l'état du serveur
   - Affiche le terrain ASCII
   - Envoie position raquette

## 🚀 Commandes de Build

```bash
# Vérifier le code
cargo check

# Compiler (debug)
cargo build

# Compiler (optimisé/rapide)
cargo build --release

# Exécuter le serveur
cargo run --release --bin server

# Exécuter le client
cargo run --release --bin client
```

## 📊 Caractéristiques Implémentées

✅ Physique réaliste (rebonds, collisions)
✅ Synchronisation réseau en temps réel
✅ 2 joueurs (extensible à 4 facilement)
✅ Affichage ASCII avec bordures Unicode
✅ Système de score automatique
✅ Gestion complète des entrées clavier
✅ Architecture client/serveur stable

## 🎨 Graphiques

- **Balle**: Cercle Unicode `●`
- **Raquettes**: Blocs `█` (3 caractères de haut)
- **Bordures**: Lignes et coins Unicode (`╔╗╚╝═║`)
- **Score**: Affichage en bas de l'écran

## 🔐 Sécurité & Stabilité

- ✓ Pas de connexion persistante requise (UDP stateless)
- ✓ Validation des positions (clamp)
- ✓ Gestion des timeouts
- ✓ Pas d'allocation dynamique excessive
- ✓ Mutex pour synchronisation thread-safe

## 📝 Améliorations Possibles

- [ ] 4 joueurs (Pong aux 4 coins)
- [ ] Niveaux de difficulté
- [ ] Power-ups dans le jeu
- [ ] Replay/historique des matchs
- [ ] Interface graphique (raylib/ggez)
- [ ] Persistence des scores
- [ ] Mode solo (IA)
- [ ] Chat intégré

---

**Version**: 0.1.0 | **Créé le**: 2026-04-07 | **Licence**: MIT
