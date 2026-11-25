use std::path::PathBuf;

use pokelauncher_core::config::Config;
use pokelauncher_core::scan::{self, ResultatScan};
use pokelauncher_core::Catalogue;
use tauri::Manager;

/// Les erreurs remontent à l'interface sous forme de texte en français.
type Resultat<T> = Result<T, String>;

fn fichier_config(app: &tauri::AppHandle) -> Resultat<PathBuf> {
    app.path()
        .app_config_dir()
        .map(|d| d.join("config.json"))
        .map_err(|e| format!("dossier de configuration introuvable : {e}"))
}

#[tauri::command]
fn catalogue() -> Catalogue {
    Catalogue::embarque()
}

#[tauri::command]
fn lire_config(app: tauri::AppHandle) -> Resultat<Config> {
    Config::charger(&fichier_config(&app)?).map_err(|e| format!("lecture des réglages impossible : {e}"))
}

#[tauri::command]
fn enregistrer_config(app: tauri::AppHandle, config: Config) -> Resultat<()> {
    config
        .enregistrer(&fichier_config(&app)?)
        .map_err(|e| format!("enregistrement des réglages impossible : {e}"))
}

/// Le hash des grosses ROMs DS prend du temps : on scanne hors du thread principal.
#[tauri::command]
async fn scanner_roms(dossier: PathBuf) -> Resultat<ResultatScan> {
    tauri::async_runtime::spawn_blocking(move || scan::scanner(&Catalogue::embarque(), &dossier))
        .await
        .map_err(|e| format!("le scan s'est interrompu : {e}"))?
        .map_err(|e| format!("scan impossible : {e}"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![catalogue, lire_config, enregistrer_config, scanner_roms])
        .run(tauri::generate_context!())
        .expect("erreur au lancement de PokéLauncher");
}
