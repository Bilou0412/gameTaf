# 🚀 Configuration GitHub pour Termigame Pong

Guide pour héberger le projet sur GitHub avec installation en une ligne.

## Étape 1: Créer le repo GitHub

1. Allez sur [GitHub](https://github.com/new)
2. Remplissez les infos:
   - **Repository name**: `termigame-pong`
   - **Description**: "🎮 Un jeu Pong multijoueur en terminal sur UDP"
   - **Public** (pour accessible partout)
3. Cliquez **"Create repository"**

## Étape 2: Push du code

```bash
cd ~/gameTaf
git remote add origin https://github.com/VOTRE_USERNAME/termigame-pong.git
git branch -M main
git push -u origin main
```

Remplacez `VOTRE_USERNAME` par votre pseudo GitHub.

## Étape 3: Mettre à jour les URLs

Remplacez `USER` par votre username dans ces fichiers:

### [install.sh](install.sh)
```bash
git clone https://github.com/VOTRE_USERNAME/termigame-pong.git "$INSTALL_DIR"
curl -fsSL "https://github.com/VOTRE_USERNAME/termigame-pong/archive/main.zip"
```

### [INSTALL.md](INSTALL.md)
```bash
curl -fsSL https://raw.githubusercontent.com/VOTRE_USERNAME/termigame-pong/main/install.sh | bash
```

### [README.md](README.md)
```markdown
[Releases](https://github.com/VOTRE_USERNAME/termigame-pong/releases)
```

## Étape 4: Créer une Release

1. Allez sur votre repo GitHub
2. Cliquez sur **"Releases"** (ou "Create a release")
3. Créez le tag `v0.1.0`
4. Titre: "🎮 v0.1.0 - Premier lancement!"
5. Description:
```
🎉 Installation maintenant disponible en une ligne!

## Installation rapide

\`\`\`bash
curl -fsSL https://raw.githubusercontent.com/VOTRE_USERNAME/termigame-pong/main/install.sh | bash
\`\`\`

## Démarrage

\`\`\`bash
termigame-pong
\`\`\`

Choisissez serveur ou client dans le menu interactif!
```

6. Cliquez **"Publish release"**

## Étape 5: Configurer GitHub Actions (Optionnel)

Le fichier `.github/workflows/release.yml` compile automatiquement les binaires pour:
- Linux x64
- macOS x64
- macOS ARM64 (M1/M2)

**Les binaires seront attachés à chaque Release! ✨**

### Pour que ça marche:
1. Push le fichier `release.yml` vers main
2. Créez une Release avec tag `v0.1.0`
3. GitHub compilera tout automatiquement

## Étape 6: Partager!

Votre lien à partager :
```
curl -fsSL https://raw.githubusercontent.com/VOTRE_USERNAME/termigame-pong/main/install.sh | bash
```

Ou sur Twitter: 
> "🎮 Just launched Termigame Pong! One-liner install: curl -fsSL ... | bash"

---

## Structure du projet

```
termigame-pong/
├── install.sh              # ← Script d'installation principal
├── INSTALL.md              # Installation detaillée
├── README.md               # Documentation utilisateur
├── GITHUB_SETUP.md         # Ce fichier
├── Cargo.toml
├── src/
│   ├── bin/
│   │   ├── client.rs
│   │   └── server.rs
│   └── lib/
│       ├── mod.rs
│       ├── game.rs
│       ├── renderer.rs
│       └── input.rs
└── .github/
    └── workflows/
        └── release.yml     # ← Compilation automatique
```

---

## Commandes utiles

```bash
# Voir l'installation localement
bash install.sh

# Simuler le one-liner
bash < <(curl -fsSL file:///home/bilel/gameTaf/install.sh)

# Vérifier syntax du workflow
# GitHub valide automatiquement lors du push
```

---

## Dépannage

**Q: "Erreur: git clone failed"**
- Assurez-vous que le repo est public
- Vérifiez l'URL est correcte

**Q: "Les binaires ne se compilent pas"**
- Vérifiez que Rust 1.70+ est utilisé
- Regardez les logs de GitHub Actions

**Q: "L'install.sh ne marche pas localement"**
```bash
bash -x install.sh  # Mode debug
```

---

## Next Steps

1. ✅ Push le code vers GitHub
2. ✅ Testez l'installation en local
3. ✅ Créez une première Release v0.1.0
4. ✅ Partagez le lien one-liner
5. ✅ Attendez les stars ⭐

Bonne chance! 🚀
