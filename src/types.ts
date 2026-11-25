// Types partagés avec le backend Rust (voir src-tauri/core).
import type catalogueJson from "../data/games.json";

export type Catalogue = typeof catalogueJson;
export type Jeu = Catalogue["jeux"][number];
export type Generation = Catalogue["generations"][number];

export type Methode = "hash" | "entete" | "inconnue";

export interface InfosEntete {
  plateforme: "gb" | "gbc" | "gba" | "nds";
  titre: string;
  code: string | null;
  langue: string | null;
}

export interface RomTrouvee {
  chemin: string;
  nomFichier: string;
  taille: number;
  crc32: string;
  sha1: string;
  entete: InfosEntete | null;
  jeuId: string | null;
  langue: string | null;
  revision: string | null;
  methode: Methode;
}

export interface ResultatScan {
  dossier: string;
  roms: RomTrouvee[];
  ignores: { chemin: string; raison: string }[];
}

export interface Config {
  dossierRoms: string | null;
}
