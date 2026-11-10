# Roadmap

## Maintenant

- [x] Maquette de l'interface (bibliothèque + fiche jeu + réglages), voir `docs/maquette/`
- [ ] Valider la maquette et lister les modifs
- [ ] Choisir la stack : Tauri vs Electron
- [ ] Scaffolder l'app

## Ensuite (V1)

- [ ] Scan du dossier ROMs + table des hash des 20 versions (`data/games.json`)
- [ ] Bibliothèque classée par génération
- [ ] Installation et mise à jour auto de mGBA et melonDS
- [ ] Lancement en un clic avec la config par console
- [ ] Sauvegardes : un dossier par jeu et par variante, backup auto avant chaque session

## Plus tard (bonus)

- [ ] Synchro des sauvegardes sur le NAS
- [ ] Progression lue dans la sauvegarde (badges, Pokédex, temps de jeu)
- [ ] Variantes : appliquer un patch (IPS / BPS avec Flips, xdelta avec xdelta3) sur une copie de la ROM, original intact
- [ ] Vérification du hash de la ROM de base attendue par un patch, avec message clair (ex. ROM FR alors que le patch attend la US)
- [ ] Bouton « Randomizer » (Universal Pokémon Randomizer ZX)
- [ ] Bouton « Ouvrir dans PKHeX » sur une sauvegarde
- [ ] Packs de textures HD / upscaling pour melonDS
- [ ] Spin-offs : Donjon Mystère, Ranger, Pinball

## Points d'attention

- Un patch ne marche que sur la version exacte de la ROM qu'il attend, presque toujours la version US, parfois une révision précise.
- Une sauvegarde du jeu original n'est pas compatible avec une version patchée : chaque variante a ses propres sauvegardes.
- Patchs, ROMs et ROMs patchées restent hors du repo.
