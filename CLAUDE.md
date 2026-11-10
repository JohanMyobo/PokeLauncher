# CLAUDE.md

Contexte pour Claude Code sur ce repo.

## Projet

PokéLauncher : lanceur desktop perso pour les jeux Pokémon principaux, Gen 1 à 5 (20 versions, GB → DS). Projet personnel, non lucratif. Cible : Mac Mini (macOS), Windows en second.

Fiche projet de référence : Notion, page « PokéLauncher » (base Projets). La roadmap vit dans `ROADMAP.md`.

## Règles non négociables

- Le lanceur ne télécharge jamais de jeux. Il lit uniquement les ROMs déposées par l'utilisateur dans un dossier local.
- Aucune ROM, aucun patch, aucune ROM patchée, aucune sauvegarde dans le repo (voir `.gitignore`).
- Appliquer un patch se fait toujours sur une copie : la ROM d'origine reste intacte.
- Chaque variante (original, romhack, randomizer) a son propre dossier de sauvegardes.
- Backup de la sauvegarde avant chaque lancement.

## Choix techniques

- Émulateurs : mGBA (GB / GBC / GBA), melonDS (DS), téléchargés et mis à jour par le lanceur.
- Identification des ROMs par hash (CRC32 + SHA-1) contre une table `data/games.json`.
- Stack à confirmer : Tauri recommandé (léger, Rust côté système de fichiers et lancement de process).

## Conventions

- Interface et textes en français.
- Commits en français, courts et descriptifs.
- La maquette de référence est dans `docs/maquette/` (fichiers `.dc.html`).
