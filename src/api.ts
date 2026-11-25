// Appels au backend Tauri. Hors Tauri (navigateur, `npm run dev`), on renvoie
// des valeurs vides pour pouvoir travailler l'interface seule.
import { invoke, isTauri } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { Config, ResultatScan } from "./types";

export const dansTauri = isTauri();

export async function lireConfig(): Promise<Config> {
  if (!dansTauri) return { dossierRoms: null };
  return invoke<Config>("lire_config");
}

export async function enregistrerConfig(config: Config): Promise<void> {
  if (!dansTauri) return;
  await invoke("enregistrer_config", { config });
}

export async function scannerRoms(dossier: string): Promise<ResultatScan> {
  if (!dansTauri) throw new Error("Le scan n'est disponible que dans l'application.");
  return invoke<ResultatScan>("scanner_roms", { dossier });
}

export async function choisirDossier(): Promise<string | null> {
  if (!dansTauri) return null;
  const choix = await open({ directory: true, multiple: false, title: "Dossier des ROMs" });
  return typeof choix === "string" ? choix : null;
}
