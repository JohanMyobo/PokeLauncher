//! Scanne un dossier et affiche le résultat en JSON.
//!
//! Utile pour relever le CRC32 et le SHA-1 de ses propres dumps et compléter `data/games.json` :
//!   cargo run -p pokelauncher-core --example scan -- ~/ROMs

fn main() {
    let Some(dossier) = std::env::args().nth(1) else {
        eprintln!("usage : scan <dossier des ROMs>");
        std::process::exit(2);
    };
    let catalogue = pokelauncher_core::Catalogue::embarque();
    match pokelauncher_core::scan::scanner(&catalogue, dossier.as_ref()) {
        Ok(resultat) => println!("{}", serde_json::to_string_pretty(&resultat).unwrap()),
        Err(e) => {
            eprintln!("scan impossible : {e}");
            std::process::exit(1);
        }
    }
}
