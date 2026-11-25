//! Table des 20 versions, chargée depuis `data/games.json`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Contenu de `data/games.json`, embarqué dans le binaire.
pub const GAMES_JSON: &str = include_str!("../../../data/games.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalogue {
    pub version: u32,
    pub consoles: BTreeMap<String, Console>,
    pub generations: Vec<Generation>,
    pub jeux: Vec<Jeu>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Console {
    pub nom: String,
    pub court: String,
    pub emulateur: String,
    pub extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generation {
    pub id: u8,
    pub titre: String,
    pub court: String,
    pub console: String,
    pub annees: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jeu {
    pub id: String,
    pub nom: String,
    pub generation: u8,
    pub console: String,
    pub annee: u16,
    pub couleur: String,
    pub encre: String,
    pub entete: CritereEntete,
    /// Dumps connus, identifiés par hash. À compléter avec ses propres cartouches.
    #[serde(default)]
    pub dumps: Vec<Dump>,
}

/// Comment reconnaître le jeu à partir de l'entête de la ROM.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CritereEntete {
    /// Titres interne exacts (Game Boy, Gen 1 : pas de code jeu).
    #[serde(default)]
    pub titres: Vec<String>,
    /// Code jeu sur 3 lettres, sans la lettre de région (GBC, GBA, DS).
    #[serde(default)]
    pub codes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dump {
    pub sha1: String,
    #[serde(default)]
    pub crc32: Option<String>,
    pub langue: String,
    #[serde(default)]
    pub revision: Option<String>,
}

impl Catalogue {
    /// Catalogue embarqué dans le binaire.
    pub fn embarque() -> Self {
        Self::depuis_json(GAMES_JSON).expect("data/games.json invalide")
    }

    pub fn depuis_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn jeu(&self, id: &str) -> Option<&Jeu> {
        self.jeux.iter().find(|j| j.id == id)
    }

    /// Toutes les extensions de fichier reconnues, en minuscules.
    pub fn extensions(&self) -> Vec<String> {
        let mut ext: Vec<String> = self
            .consoles
            .values()
            .flat_map(|c| c.extensions.iter().map(|e| e.to_lowercase()))
            .collect();
        ext.sort();
        ext.dedup();
        ext
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_catalogue_embarque_contient_les_20_versions() {
        let c = Catalogue::embarque();
        assert_eq!(c.jeux.len(), 20);
        assert_eq!(c.generations.len(), 5);
        for jeu in &c.jeux {
            assert!(c.consoles.contains_key(&jeu.console), "console inconnue pour {}", jeu.id);
            assert!(
                c.generations.iter().any(|g| g.id == jeu.generation),
                "génération inconnue pour {}",
                jeu.id
            );
            assert!(
                !jeu.entete.titres.is_empty() || !jeu.entete.codes.is_empty(),
                "aucun critère d'entête pour {}",
                jeu.id
            );
        }
    }

    #[test]
    fn les_identifiants_sont_uniques() {
        let c = Catalogue::embarque();
        let mut ids: Vec<_> = c.jeux.iter().map(|j| j.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 20);
    }
}
