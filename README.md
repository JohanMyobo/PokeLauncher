# PokéLauncher

Lanceur perso, dans l'esprit des lanceurs « All-in-One », pour les jeux Pokémon principaux de la Gen 1 (Game Boy) à la Gen 5 (Nintendo DS). Une bibliothèque visuelle, lancement en un clic, émulateurs gérés, sauvegardes protégées et variantes (romhacks, randomizer).

Projet personnel, non lucratif.

> **Règle de base : le lanceur ne télécharge jamais de jeux.** Il détecte les ROMs extraites de mes propres cartouches, déposées dans un dossier local. Aucune ROM, aucun patch et aucune sauvegarde ne doit être commité dans ce repo.

## Les 20 versions

| Génération | Console | Versions |
|---|---|---|
| Gen 1 | Game Boy | Rouge, Bleu, Jaune |
| Gen 2 | Game Boy Color | Or, Argent, Cristal |
| Gen 3 | Game Boy Advance | Rubis, Saphir, Émeraude, Rouge Feu, Vert Feuille |
| Gen 4 | Nintendo DS | Diamant, Perle, Platine, HeartGold, SoulSilver |
| Gen 5 | Nintendo DS | Noir, Blanc, Noir 2, Blanc 2 |

## Fonctionnalités V1

- Bibliothèque visuelle classée par génération (jaquette, année, console)
- Détection des ROMs dans un dossier + identification par hash (CRC / SHA-1) : jeu, version, langue
- Émulateurs gérés : téléchargement et mise à jour auto de **mGBA** (GB / GBC / GBA) et **melonDS** (DS)
- Lancement en un clic avec la bonne config par console
- Sauvegardes : un dossier par jeu et par variante, backup auto avant chaque session

Bonus envisagés : synchro NAS, progression lue dans la sauvegarde, romhacks (IPS / BPS / xdelta), randomizer, PKHeX, packs HD pour melonDS. Détails dans [ROADMAP.md](ROADMAP.md).

## Maquette

La maquette de l'interface (bibliothèque, fiche jeu, réglages) est dans [`docs/maquette/`](docs/maquette/).

## Stack

**Tauri 2** (backend Rust) + **React / TypeScript** (Vite). Cible principale : le Mac Mini, Windows en second.

## Développement

Prérequis : Node 20+, Rust stable ([rustup](https://rustup.rs)) et, sur macOS, les outils Xcode en ligne de commande (`xcode-select --install`).

```bash
npm install
npm run tauri dev      # lance l'app en mode développement
npm run tauri build    # construit l'app (.app / .dmg sur macOS)
```

Tests du cœur (scan, entêtes, catalogue), sans interface :

```bash
cd src-tauri && cargo test -p pokelauncher-core
```

Relever le CRC32 et le SHA-1 de ses ROMs pour compléter `data/games.json` :

```bash
cd src-tauri && cargo run -p pokelauncher-core --example scan -- ~/ROMs
```

## Identification des ROMs

1. **Par hash** : si le SHA-1 du fichier figure dans `dumps` de `data/games.json`, la ROM est « vérifiée » (jeu, langue, révision).
2. **Par entête** : sinon, le lanceur lit le code jeu dans l'entête (ex. `CPUF` = Platine FR) ou, pour la Gen 1, le titre interne. La langue vient de la lettre de région ; pour la Gen 1 elle reste inconnue tant que le hash n'est pas renseigné.

Les `dumps` sont vides au départ : on les complète avec ses propres cartouches.

## Arborescence prévue

```
PokeLauncher/
├── docs/            # maquette, notes de conception
├── src/             # interface React (bibliothèque, fiche, réglages)
├── src-tauri/       # app Tauri : commandes exposées à l'interface
│   └── core/        # cœur Rust sans Tauri : catalogue, entêtes, scan, config
└── data/
    └── games.json   # table des 20 versions (couleurs, critères d'entête, hash connus)
```

Les dossiers de données utilisateur (ROMs, sauvegardes, patchs, émulateurs téléchargés) vivent hors du repo.
