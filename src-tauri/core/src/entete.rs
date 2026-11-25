//! Lecture des entêtes de ROM (Game Boy, Game Boy Color, GBA, DS).

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Plateforme {
    Gb,
    Gbc,
    Gba,
    Nds,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfosEntete {
    pub plateforme: Plateforme,
    /// Titre interne (ex. « POKEMON RED », « POKEMON PL »).
    pub titre: String,
    /// Code jeu complet sur 4 caractères, région incluse (ex. « CPUF »).
    pub code: Option<String>,
    /// Langue déduite de la région, si possible (« FR », « EN », « JP »…).
    pub langue: Option<String>,
}

impl InfosEntete {
    /// Code jeu sans la lettre de région (ex. « CPU »).
    pub fn code_jeu(&self) -> Option<&str> {
        self.code.as_deref().and_then(|c| c.get(..3))
    }
}

/// Lit l'entête selon l'extension du fichier. `None` si le fichier est trop court
/// ou ne ressemble pas à une ROM de cette plateforme.
pub fn lire(extension: &str, octets: &[u8]) -> Option<InfosEntete> {
    match extension.to_lowercase().as_str() {
        "gb" | "gbc" => lire_gb(octets),
        "gba" => lire_gba(octets),
        "nds" => lire_nds(octets),
        _ => None,
    }
}

/// Taille minimale à lire au début du fichier pour décoder n'importe quelle entête.
pub const TAILLE_ENTETE: usize = 0x150;

fn texte(octets: &[u8]) -> String {
    octets
        .iter()
        .take_while(|&&b| b != 0)
        .map(|&b| if b.is_ascii_graphic() || b == b' ' { b as char } else { ' ' })
        .collect::<String>()
        .trim()
        .to_string()
}

fn code_valide(code: &[u8]) -> bool {
    code.len() == 4 && code.iter().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
}

/// Langue associée à la lettre de région d'un code jeu Nintendo.
pub fn langue_region(lettre: char) -> Option<&'static str> {
    match lettre {
        'F' => Some("FR"),
        'E' | 'P' => Some("EN"),
        'J' => Some("JP"),
        'D' => Some("DE"),
        'I' => Some("IT"),
        'S' => Some("ES"),
        'K' => Some("KO"),
        _ => None,
    }
}

fn avec_code(plateforme: Plateforme, titre: String, code: &[u8]) -> InfosEntete {
    let code = code_valide(code).then(|| String::from_utf8_lossy(code).into_owned());
    let langue = code
        .as_deref()
        .and_then(|c| c.chars().nth(3))
        .and_then(langue_region)
        .map(str::to_string);
    InfosEntete { plateforme, titre, code, langue }
}

fn lire_gb(o: &[u8]) -> Option<InfosEntete> {
    if o.len() < 0x150 {
        return None;
    }
    // Logo Nintendo : les premiers octets sont toujours CE ED 66 66.
    if o[0x104..0x108] != [0xCE, 0xED, 0x66, 0x66] {
        return None;
    }
    let cgb = o[0x143] & 0x80 != 0;
    let code = &o[0x13F..0x143];
    if cgb && code_valide(code) {
        // Cartouche Game Boy Color récente : titre sur 11 octets puis code jeu.
        return Some(avec_code(Plateforme::Gbc, texte(&o[0x134..0x13F]), code));
    }
    let titre = texte(&o[0x134..0x144]);
    // Code destination : 0x00 = Japon, sinon international (langue non déterminable).
    let langue = (o[0x14A] == 0x00).then(|| "JP".to_string());
    let plateforme = if cgb { Plateforme::Gbc } else { Plateforme::Gb };
    Some(InfosEntete { plateforme, titre, code: None, langue })
}

fn lire_gba(o: &[u8]) -> Option<InfosEntete> {
    if o.len() < 0xC0 || o[0xB2] != 0x96 {
        return None;
    }
    Some(avec_code(Plateforme::Gba, texte(&o[0xA0..0xAC]), &o[0xAC..0xB0]))
}

fn lire_nds(o: &[u8]) -> Option<InfosEntete> {
    if o.len() < 0x10 {
        return None;
    }
    let code = &o[0x0C..0x10];
    if !code_valide(code) {
        return None;
    }
    Some(avec_code(Plateforme::Nds, texte(&o[0x00..0x0C]), code))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Fabrique une fausse ROM Game Boy (aucune vraie donnée de jeu).
    pub fn fausse_gb(titre: &str, code: Option<&str>, cgb: u8, destination: u8) -> Vec<u8> {
        let mut o = vec![0u8; 0x8000];
        o[0x104..0x108].copy_from_slice(&[0xCE, 0xED, 0x66, 0x66]);
        o[0x134..0x134 + titre.len()].copy_from_slice(titre.as_bytes());
        if let Some(c) = code {
            o[0x13F..0x143].copy_from_slice(c.as_bytes());
        }
        o[0x143] = cgb;
        o[0x14A] = destination;
        o
    }

    pub fn fausse_gba(titre: &str, code: &str) -> Vec<u8> {
        let mut o = vec![0u8; 0x200];
        o[0xA0..0xA0 + titre.len()].copy_from_slice(titre.as_bytes());
        o[0xAC..0xB0].copy_from_slice(code.as_bytes());
        o[0xB2] = 0x96;
        o
    }

    pub fn fausse_nds(titre: &str, code: &str) -> Vec<u8> {
        let mut o = vec![0u8; 0x200];
        o[..titre.len()].copy_from_slice(titre.as_bytes());
        o[0x0C..0x10].copy_from_slice(code.as_bytes());
        o
    }

    #[test]
    fn gen1_titre_sans_langue() {
        let e = lire("gb", &fausse_gb("POKEMON RED", None, 0x00, 0x01)).unwrap();
        assert_eq!(e.plateforme, Plateforme::Gb);
        assert_eq!(e.titre, "POKEMON RED");
        assert_eq!(e.code, None);
        assert_eq!(e.langue, None);
    }

    #[test]
    fn gen1_japonaise() {
        let e = lire("gb", &fausse_gb("POKEMON GREEN", None, 0x00, 0x00)).unwrap();
        assert_eq!(e.langue.as_deref(), Some("JP"));
    }

    #[test]
    fn gen2_code_et_langue() {
        let e = lire("gbc", &fausse_gb("PM_CRYSTAL", Some("BYTF"), 0xC0, 0x01)).unwrap();
        assert_eq!(e.plateforme, Plateforme::Gbc);
        assert_eq!(e.titre, "PM_CRYSTAL");
        assert_eq!(e.code.as_deref(), Some("BYTF"));
        assert_eq!(e.code_jeu(), Some("BYT"));
        assert_eq!(e.langue.as_deref(), Some("FR"));
    }

    #[test]
    fn gba_et_ds() {
        let e = lire("gba", &fausse_gba("POKEMON EMER", "BPEF")).unwrap();
        assert_eq!((e.code_jeu(), e.langue.as_deref()), (Some("BPE"), Some("FR")));
        let e = lire("nds", &fausse_nds("POKEMON PL", "CPUE")).unwrap();
        assert_eq!((e.code_jeu(), e.langue.as_deref()), (Some("CPU"), Some("EN")));
    }

    #[test]
    fn fichier_invalide() {
        assert!(lire("gb", &[0u8; 0x200]).is_none());
        assert!(lire("gba", &[0u8; 0x10]).is_none());
        assert!(lire("nds", &[0u8; 0x200]).is_none());
        assert!(lire("txt", &[0u8; 0x200]).is_none());
    }
}
