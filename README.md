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

À choisir : **Tauri** (léger, Mac + Windows, recommandé) ou Electron. Cible principale : le Mac Mini.

## Arborescence prévue

```
PokeLauncher/
├── docs/            # maquette, notes de conception
├── src/             # interface (à scaffolder)
├── src-tauri/       # backend Rust si Tauri (à scaffolder)
└── data/
    └── games.json   # table des 20 versions + hash attendus (à venir)
```

Les dossiers de données utilisateur (ROMs, sauvegardes, patchs, émulateurs téléchargés) vivent hors du repo.
