import catalogue from "../../data/games.json";
import type { RomTrouvee } from "../types";
import { Jaquette } from "./Jaquette";

interface Props {
  roms: Map<string, RomTrouvee[]>;
  recherche: string;
  dossierChoisi: boolean;
  onOuvrir: (jeuId: string) => void;
  onReglages: () => void;
}

const normaliser = (s: string) =>
  s.normalize("NFD").replace(/[̀-ͯ]/g, "").toLowerCase().trim();

export function Bibliotheque({ roms, recherche, dossierChoisi, onOuvrir, onReglages }: Props) {
  const filtre = normaliser(recherche);
  const sections = catalogue.generations
    .map((g) => ({
      g,
      jeux: catalogue.jeux.filter((j) => j.generation === g.id && normaliser(`pokemon ${j.nom}`).includes(filtre)),
    }))
    .filter((s) => s.jeux.length > 0);

  return (
    <div className="pile">
      {!dossierChoisi && (
        <section className="encart">
          <div className="pile serree">
            <div className="sur-titre">Pour commencer</div>
            <div className="titre-encart">Indiquez où sont vos ROMs</div>
            <div className="attenue">
              PokéLauncher lit uniquement les ROMs que vous déposez dans un dossier local. Rien n’est téléchargé ni modifié.
            </div>
          </div>
          <button className="bouton principal" onClick={onReglages}>
            Choisir le dossier
          </button>
        </section>
      )}

      {sections.length === 0 && <div className="attenue">Aucun jeu ne correspond à « {recherche} ».</div>}

      {sections.map(({ g, jeux }) => (
        <section key={g.id} id={`gen${g.id}`} className="pile moyenne">
          <div className="titre-section">
            <h2>{g.titre}</h2>
            <span className="discret">
              {catalogue.consoles[g.console as keyof typeof catalogue.consoles].nom} · {g.annees}
            </span>
          </div>
          <div className="grille">
            {jeux.map((jeu) => {
              const rom = roms.get(jeu.id)?.[0];
              return (
                <button key={jeu.id} className="carte" onClick={() => onOuvrir(jeu.id)}>
                  <Jaquette jeu={jeu} absent={!rom} />
                  <div className="pied-carte">
                    <span className="discret">{jeu.annee}</span>
                    {rom ? (
                      <span className="pastille ok" title={rom.methode === "hash" ? "Vérifiée par SHA-1" : rom.langue ? "Reconnue par l’entête" : "Langue inconnue tant que le SHA-1 n’est pas dans data/games.json"}>
                        {rom.langue ?? "Langue ?"}
                      </span>
                    ) : (
                      <span className="pastille manque">ROM manquante</span>
                    )}
                  </div>
                </button>
              );
            })}
          </div>
        </section>
      ))}
    </div>
  );
}
