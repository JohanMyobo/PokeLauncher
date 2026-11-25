//! Scan d'un dossier de ROMs et identification des jeux.
//!
//! Le scan ne fait que lire : aucun fichier n'est modifié, déplacé ou renommé.

use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use serde::Serialize;
use sha1::{Digest, Sha1};
use walkdir::WalkDir;

use crate::catalogue::{Catalogue, Jeu};
use crate::entete::{self, InfosEntete, TAILLE_ENTETE};

/// Au-delà, ce n'est pas une ROM GB / GBA / DS (la plus grosse ROM DS fait 512 Mo).
const TAILLE_MAX: u64 = 512 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Methode {
    /// Le SHA-1 correspond à un dump connu de `games.json`.
    Hash,
    /// Reconnu par l'entête (titre ou code jeu), hash non répertorié.
    Entete,
    /// Fichier ROM lisible mais qui ne correspond à aucun des 20 jeux.
    Inconnue,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RomTrouvee {
    pub chemin: PathBuf,
    pub nom_fichier: String,
    pub taille: u64,
    pub crc32: String,
    pub sha1: String,
    pub entete: Option<InfosEntete>,
    pub jeu_id: Option<String>,
    pub langue: Option<String>,
    pub revision: Option<String>,
    pub methode: Methode,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultatScan {
    pub dossier: PathBuf,
    pub roms: Vec<RomTrouvee>,
    /// Fichiers ignorés (illisibles, trop gros…), avec la raison.
    pub ignores: Vec<FichierIgnore>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FichierIgnore {
    pub chemin: PathBuf,
    pub raison: String,
}

impl ResultatScan {
    /// Ids des jeux du catalogue pour lesquels au moins une ROM a été trouvée.
    pub fn jeux_reconnus(&self) -> Vec<&str> {
        let mut ids: Vec<&str> = self.roms.iter().filter_map(|r| r.jeu_id.as_deref()).collect();
        ids.sort();
        ids.dedup();
        ids
    }
}

/// Calcule CRC32 et SHA-1 en une seule lecture, et renvoie aussi le début du fichier.
fn hacher(chemin: &Path) -> io::Result<(String, String, Vec<u8>)> {
    let mut fichier = File::open(chemin)?;
    let mut crc = crc32fast::Hasher::new();
    let mut sha = Sha1::new();
    let mut debut = Vec::with_capacity(TAILLE_ENTETE);
    let mut tampon = vec![0u8; 1024 * 1024];
    loop {
        let n = fichier.read(&mut tampon)?;
        if n == 0 {
            break;
        }
        if debut.len() < TAILLE_ENTETE {
            let manque = (TAILLE_ENTETE - debut.len()).min(n);
            debut.extend_from_slice(&tampon[..manque]);
        }
        crc.update(&tampon[..n]);
        sha.update(&tampon[..n]);
    }
    let sha1 = sha.finalize().iter().map(|b| format!("{b:02x}")).collect();
    Ok((format!("{:08x}", crc.finalize()), sha1, debut))
}

fn par_entete<'a>(catalogue: &'a Catalogue, e: &InfosEntete) -> Option<&'a Jeu> {
    catalogue.jeux.iter().find(|j| {
        let code_ok = e.code_jeu().is_some_and(|c| j.entete.codes.iter().any(|x| x == c));
        let titre_ok = e.code.is_none() && j.entete.titres.iter().any(|t| *t == e.titre);
        code_ok || titre_ok
    })
}

/// Identifie une ROM à partir de son contenu (hash d'abord, entête ensuite).
pub fn identifier(catalogue: &Catalogue, chemin: &Path) -> io::Result<RomTrouvee> {
    let taille = std::fs::metadata(chemin)?.len();
    let (crc32, sha1, debut) = hacher(chemin)?;
    let extension = chemin.extension().and_then(|e| e.to_str()).unwrap_or_default();
    let entete = entete::lire(extension, &debut);

    let mut rom = RomTrouvee {
        chemin: chemin.to_path_buf(),
        nom_fichier: chemin.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(),
        taille,
        crc32,
        sha1,
        langue: entete.as_ref().and_then(|e| e.langue.clone()),
        entete,
        jeu_id: None,
        revision: None,
        methode: Methode::Inconnue,
    };

    let par_hash = catalogue.jeux.iter().find_map(|j| {
        j.dumps.iter().find(|d| d.sha1.eq_ignore_ascii_case(&rom.sha1)).map(|d| (j, d))
    });
    if let Some((jeu, dump)) = par_hash {
        rom.jeu_id = Some(jeu.id.clone());
        rom.langue = Some(dump.langue.clone());
        rom.revision = dump.revision.clone();
        rom.methode = Methode::Hash;
    } else if let Some(jeu) = rom.entete.as_ref().and_then(|e| par_entete(catalogue, e)) {
        rom.jeu_id = Some(jeu.id.clone());
        rom.methode = Methode::Entete;
    }
    Ok(rom)
}

/// Parcourt le dossier (sous-dossiers compris) et identifie chaque ROM.
pub fn scanner(catalogue: &Catalogue, dossier: &Path) -> io::Result<ResultatScan> {
    if !dossier.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("le dossier {} n'existe pas", dossier.display()),
        ));
    }
    let extensions = catalogue.extensions();
    let mut resultat = ResultatScan { dossier: dossier.to_path_buf(), roms: vec![], ignores: vec![] };

    for entree in WalkDir::new(dossier).follow_links(true).into_iter().filter_map(Result::ok) {
        let chemin = entree.path();
        let ext = chemin.extension().and_then(|e| e.to_str()).map(str::to_lowercase);
        if !entree.file_type().is_file() || !ext.is_some_and(|e| extensions.contains(&e)) {
            continue;
        }
        if entree.metadata().map(|m| m.len() > TAILLE_MAX).unwrap_or(false) {
            resultat.ignores.push(FichierIgnore { chemin: chemin.into(), raison: "fichier trop gros".into() });
            continue;
        }
        match identifier(catalogue, chemin) {
            Ok(rom) => resultat.roms.push(rom),
            Err(e) => resultat.ignores.push(FichierIgnore { chemin: chemin.into(), raison: e.to_string() }),
        }
    }
    resultat.roms.sort_by(|a, b| a.chemin.cmp(&b.chemin));
    Ok(resultat)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalogue::Dump;
    use crate::entete::tests::{fausse_gb, fausse_gba, fausse_nds};
    use std::fs;

    #[test]
    fn scan_identifie_par_entete() {
        let dossier = tempfile::tempdir().unwrap();
        let sous = dossier.path().join("DS");
        fs::create_dir(&sous).unwrap();
        fs::write(dossier.path().join("rouge.gb"), fausse_gb("POKEMON RED", None, 0, 1)).unwrap();
        fs::write(dossier.path().join("or.gbc"), fausse_gb("POKEMON_GLD", Some("AAUF"), 0x80, 1)).unwrap();
        fs::write(dossier.path().join("emeraude.GBA"), fausse_gba("POKEMON EMER", "BPEF")).unwrap();
        fs::write(sous.join("platine.nds"), fausse_nds("POKEMON PL", "CPUF")).unwrap();
        fs::write(dossier.path().join("autre.gba"), fausse_gba("ZELDA", "BZME")).unwrap();
        fs::write(dossier.path().join("notes.txt"), "pas une rom").unwrap();

        let c = Catalogue::embarque();
        let r = scanner(&c, dossier.path()).unwrap();
        assert_eq!(r.roms.len(), 5);
        assert_eq!(r.jeux_reconnus(), vec!["emeraude", "or", "platine", "rouge"]);

        let platine = r.roms.iter().find(|x| x.jeu_id.as_deref() == Some("platine")).unwrap();
        assert_eq!(platine.methode, Methode::Entete);
        assert_eq!(platine.langue.as_deref(), Some("FR"));

        let autre = r.roms.iter().find(|x| x.nom_fichier == "autre.gba").unwrap();
        assert_eq!(autre.methode, Methode::Inconnue);
        assert_eq!(autre.jeu_id, None);
    }

    #[test]
    fn le_hash_prime_sur_l_entete() {
        let dossier = tempfile::tempdir().unwrap();
        let chemin = dossier.path().join("bleu.gb");
        fs::write(&chemin, fausse_gb("POKEMON BLUE", None, 0, 1)).unwrap();

        let mut c = Catalogue::embarque();
        let sha1 = identifier(&c, &chemin).unwrap().sha1;
        c.jeux.iter_mut().find(|j| j.id == "bleu").unwrap().dumps.push(Dump {
            sha1: sha1.to_uppercase(),
            crc32: None,
            langue: "FR".into(),
            revision: None,
        });

        let rom = identifier(&c, &chemin).unwrap();
        assert_eq!(rom.methode, Methode::Hash);
        assert_eq!(rom.jeu_id.as_deref(), Some("bleu"));
        assert_eq!(rom.langue.as_deref(), Some("FR"));
    }

    #[test]
    fn hash_connus() {
        let dossier = tempfile::tempdir().unwrap();
        let chemin = dossier.path().join("vide.gba");
        fs::write(&chemin, b"abc").unwrap();
        let (crc, sha, _) = hacher(&chemin).unwrap();
        assert_eq!(crc, "352441c2");
        assert_eq!(sha, "a9993e364706816aba3e25717850c26c9cd0d89d");
    }

    #[test]
    fn dossier_absent() {
        let c = Catalogue::embarque();
        assert!(scanner(&c, Path::new("/n/existe/pas")).is_err());
    }
}
