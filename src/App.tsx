import { useCallback, useEffect, useMemo, useState } from "react";
import catalogue from "../data/games.json";
import { choisirDossier, dansTauri, enregistrerConfig, lireConfig, scannerRoms } from "./api";
import type { Config, ResultatScan, RomTrouvee } from "./types";
import { Bibliotheque } from "./components/Bibliotheque";
import { Fiche } from "./components/Fiche";
import { Reglages } from "./components/Reglages";
import { IconeRafraichir, IconeReglages, IconeRecherche, Logo } from "./components/Icones";

export type Vue = { ecran: "bibliotheque" } | { ecran: "reglages" } | { ecran: "fiche"; jeuId: string };

/** ROMs trouvées, regroupées par jeu ; la meilleure (hash vérifié, puis FR) en premier. */
function romsParJeu(scan: ResultatScan | null): Map<string, RomTrouvee[]> {
  const parJeu = new Map<string, RomTrouvee[]>();
  for (const rom of scan?.roms ?? []) {
    if (!rom.jeuId) continue;
    parJeu.set(rom.jeuId, [...(parJeu.get(rom.jeuId) ?? []), rom]);
  }
  const score = (r: RomTrouvee) => (r.methode === "hash" ? 2 : 0) + (r.langue === "FR" ? 1 : 0);
  for (const roms of parJeu.values()) roms.sort((a, b) => score(b) - score(a));
  return parJeu;
}

export default function App() {
  const [vue, setVue] = useState<Vue>({ ecran: "bibliotheque" });
  const [config, setConfig] = useState<Config>({ dossierRoms: null });
  const [scan, setScan] = useState<ResultatScan | null>(null);
  const [enCours, setEnCours] = useState(false);
  const [erreur, setErreur] = useState<string | null>(null);
  const [recherche, setRecherche] = useState("");

  const lancerScan = useCallback(async (dossier: string | null) => {
    if (!dossier) {
      setVue({ ecran: "reglages" });
      return;
    }
    setEnCours(true);
    setErreur(null);
    try {
      setScan(await scannerRoms(dossier));
    } catch (e) {
      setErreur(String(e));
    } finally {
      setEnCours(false);
    }
  }, []);

  useEffect(() => {
    lireConfig()
      .then((c) => {
        setConfig(c);
        if (c.dossierRoms) void lancerScan(c.dossierRoms);
      })
      .catch((e) => setErreur(String(e)));
  }, [lancerScan]);

  const changerDossier = async () => {
    const dossier = await choisirDossier();
    if (!dossier) return;
    const nouvelle = { ...config, dossierRoms: dossier };
    setConfig(nouvelle);
    try {
      await enregistrerConfig(nouvelle);
    } catch (e) {
      setErreur(String(e));
    }
    await lancerScan(dossier);
  };

  const roms = useMemo(() => romsParJeu(scan), [scan]);
  const reconnus = catalogue.jeux.filter((j) => roms.has(j.id)).length;

  return (
    <div className="app">
      <header className="entete">
        <button className="marque" onClick={() => setVue({ ecran: "bibliotheque" })}>
          <Logo />
          <span>PokéLauncher</span>
        </button>
        <label className="recherche">
          <IconeRecherche />
          <span className="masque">Rechercher un jeu</span>
          <input
            type="search"
            placeholder="Rechercher un jeu…"
            value={recherche}
            onChange={(e) => {
              setRecherche(e.target.value);
              setVue({ ecran: "bibliotheque" });
            }}
          />
        </label>
        <div className="actions">
          <button className="bouton" onClick={() => lancerScan(config.dossierRoms)} disabled={enCours}>
            <IconeRafraichir />
            {enCours ? "Scan en cours…" : "Rescanner les ROMs"}
          </button>
          <button className="bouton carre" aria-label="Réglages" onClick={() => setVue({ ecran: "reglages" })}>
            <IconeReglages />
          </button>
        </div>
      </header>

      <div className="corps">
        <nav className="menu" aria-label="Navigation">
          <div className="groupe">
            <button className={vue.ecran !== "reglages" ? "lien actif" : "lien"} onClick={() => setVue({ ecran: "bibliotheque" })}>
              Bibliothèque
            </button>
            <button className={vue.ecran === "reglages" ? "lien actif" : "lien"} onClick={() => setVue({ ecran: "reglages" })}>
              Réglages
            </button>
          </div>
          <div className="groupe">
            <div className="sur-titre">Générations</div>
            {catalogue.generations.map((g) => {
              const jeux = catalogue.jeux.filter((j) => j.generation === g.id);
              return (
                <a
                  key={g.id}
                  className="lien gen"
                  href={`#gen${g.id}`}
                  onClick={() => setVue({ ecran: "bibliotheque" })}
                >
                  <span>{g.court}</span>
                  <span className="discret">
                    {jeux.filter((j) => roms.has(j.id)).length}/{jeux.length}
                  </span>
                </a>
              );
            })}
          </div>
          <div className="compteur">
            <div className="discret petit">ROMs reconnues</div>
            <div className="chiffre">
              {reconnus} / {catalogue.jeux.length}
            </div>
            <div className="jauge">
              <div style={{ width: `${(reconnus / catalogue.jeux.length) * 100}%` }} />
            </div>
          </div>
        </nav>

        <main className="contenu">
          {!dansTauri && (
            <div className="bandeau">Aperçu navigateur : lancez l’app avec <code>npm run tauri dev</code> pour scanner vos ROMs.</div>
          )}
          {erreur && <div className="bandeau erreur">{erreur}</div>}
          {vue.ecran === "bibliotheque" && (
            <Bibliotheque
              roms={roms}
              recherche={recherche}
              dossierChoisi={!!config.dossierRoms}
              onOuvrir={(jeuId) => setVue({ ecran: "fiche", jeuId })}
              onReglages={() => setVue({ ecran: "reglages" })}
            />
          )}
          {vue.ecran === "fiche" && (
            <Fiche jeuId={vue.jeuId} roms={roms.get(vue.jeuId) ?? []} onRetour={() => setVue({ ecran: "bibliotheque" })} />
          )}
          {vue.ecran === "reglages" && (
            <Reglages config={config} scan={scan} enCours={enCours} onChoisirDossier={changerDossier} />
          )}
        </main>
      </div>
    </div>
  );
}
