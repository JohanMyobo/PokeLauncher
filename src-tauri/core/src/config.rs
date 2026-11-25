//! Réglages de l'utilisateur, stockés en JSON dans le dossier de config de l'app.

use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Config {
    /// Dossier où l'utilisateur dépose ses ROMs.
    pub dossier_roms: Option<PathBuf>,
}

impl Config {
    /// Charge la config ; renvoie la config par défaut si le fichier n'existe pas encore.
    pub fn charger(fichier: &Path) -> io::Result<Self> {
        match std::fs::read_to_string(fichier) {
            Ok(txt) => serde_json::from_str(&txt).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(e),
        }
    }

    pub fn enregistrer(&self, fichier: &Path) -> io::Result<()> {
        if let Some(parent) = fichier.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let txt = serde_json::to_string_pretty(self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        std::fs::write(fichier, txt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aller_retour() {
        let d = tempfile::tempdir().unwrap();
        let f = d.path().join("sous/config.json");
        assert_eq!(Config::charger(&f).unwrap(), Config::default());
        let c = Config { dossier_roms: Some("/Users/moi/ROMs".into()) };
        c.enregistrer(&f).unwrap();
        assert_eq!(Config::charger(&f).unwrap(), c);
    }
}
