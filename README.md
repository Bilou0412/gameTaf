# 🎮 TERMIGAME PONG - Jeu Pong Multijoueur en LAN

Un jeu Pong classique jouable en terminal via UDP sur réseau local (LAN).

## 🚀 Installation (Une ligne !)

```bash
curl -fsSL https://raw.githubusercontent.com/Bilou0412/gameTaf/main/install.sh | bash
```

### Après installation
```bash
termigame-pong
```

Un menu interactif vous permet de choisir :
- 🎮 Lancer le serveur
- 👾 Rejoindre une partie
- 📖 Afficher les contrôles

👉 **[Documentation complète d'installation](INSTALL.md)**

## 🎮 Contrôles

| Touche | Action |
|--------|--------|
| **W/Z** | Raquette vers le haut |
| **S** | Raquette vers le bas |
| **Q** | Quitter |

## 📋 Architecture

- **Serveur (server.rs)** : Gère la logique du jeu et synchronise les états
- **Client (client.rs)** : Interface joueur et communication réseau
- **Modules** :
  - `game::GameState` : Logique physique (balle, raquettes, collisions)
  - `renderer::Renderer` : Affichage ASCII du terrain
  - `input::InputHandler` : Gestion des entrées clavier

## 🔧 Configuration réseau

Le serveur écoute sur **`0.0.0.0:9999`**

Pour jouer sur différents ordinateurs:
1. Trouvez l'IP du serveur: `ipconfig` (Windows) ou `ifconfig` (Linux/Mac)
2. Lancez les clients avec cette IP

## 📊 Caractéristiques

- ✅ Pong 2 joueurs classique
- ✅ Communication UDP temps réel
- ✅ Affichage ASCII avec bordures
- ✅ Système de score automatique
- ✅ Collisions physiques réalistes
- ✅ Jeu 80x24 caractères

## 🐛 Dépannage

**Les clients ne se connectent pas?**
- Vérifiez que le serveur est bien lancé
- Vérifiez l'adresse IP (utilisez `localhost` si sur la même machine)
- Vérifiez le pare-feu (port UDP 9999)

**Affichage déformé?**
- Agrandissez votre terminal à au moins 80x24 caractères
- Utilisez un terminal supportant Unicode

## 📝 Licence

Projet libre de droits - utilisez-le comme bon vous semble!
