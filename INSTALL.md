# 🎮 TERMIGAME PONG - Installation

Installation en une ligne, simple et rapide !

## Installation rapide (One-liner)

```bash
curl -fsSL https://raw.githubusercontent.com/bilou0412/termigame-pong/main/install.sh | bash
```

---

## Après l'installation

Une fois l'installation terminée, utilisez simplement :

```bash
termigame-pong
```

Cela lance un **menu interactif** où vous pouvez choisir :
- 🎮 **Lancer le serveur**
- 👾 **Rejoindre une partie (client)**
- 📖 **Afficher les contrôles**

---

## Prérequis

### Obligatoire
- **Git** (pour le clone) - Ou `curl` en fallback
- **Rust 1.70+** (optionnel si vous avez les binaires pré-compilés)

### Installation de Rust (si nécessaire)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Contrôles du jeu

| Touche | Action |
|--------|--------|
| **W / Z** | Raquette vers le haut |
| **S** | Raquette vers le bas |
| **Q** | Quitter |

---

## Jouer au jeu

### Méthode 1: Même ordinateur (localhost)
```bash
# Terminal 1 - Serveur
termigame-pong
# Choisissez option 1

# Terminal 2 - Client 1
termigame-pong
# Choisissez option 2
# Entrez: 127.0.0.1

# Terminal 3 - Client 2
termigame-pong
# Choisissez option 2
# Entrez: 127.0.0.1
```

### Méthode 2: Sur un réseau (LAN)
```bash
# Sur l'ordinateur serveur
termigame-pong
# Choisissez option 1
# Notez l'adresse IP affichée (ex: 192.168.1.100)

# Sur les ordinateurs clients
termigame-pong
# Choisissez option 2
# Entrez l'adresse IP du serveur
```

---

## Désinstallation

```bash
rm -rf ~/.termigame-pong
```

Puis supprimez la ligne `termigame-pong` de votre `~/.zshrc` ou `~/.bashrc`.

---

## Mise à jour

Réexécutez simplement le script d'installation :
```bash
curl -fsSL https://raw.githubusercontent.com/bilou0412/termigame-pong/main/install.sh | bash
```

---

## Dépannage

**"Commande non trouvée termigame-pong"**
```bash
source ~/.zshrc
# ou
source ~/.bashrc
```

**"Erreur de compilation"**
Assurez-vous que Rust 1.70+ est installé :
```bash
rustup update
```

---

## À propos

🎮 Un classique du jeu vidéo, revisité en terminal UDP!

[Voir le projet sur GitHub](https://github.com/USER/termigame-pong)
