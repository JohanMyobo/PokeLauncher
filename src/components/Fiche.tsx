import catalogue from "../../data/games.json";
import type { RomTrouvee } from "../types";
import { IconeJouer } from "./Icones";
import { Jaquette } from "./Jaquette";

const methodes = { hash: "Vérifiée (SHA-1)", entete: "Reconnue par l’entête", inconnue: "Inconnue" };

function taille(octets: number) {
  return octets >= 1024 * 1024 ? `${(octets / 1024 / 1024).toFixed(1)} Mo` : `${Math.round(octets / 1024)} Ko`;
}

export function Fiche({ jeuId, roms, onRetour }: { jeuId: string; roms: RomTrouvee[]; onRetour: () => void }) {
  const jeu = catalogue.jeux.find((j) => j.id === jeuId);
  if (!jeu) return null;
  const console = catalogue.consoles[jeu.console as keyof typeof catalogue.consoles];

  return (
    <div className="pile">
      <button className="retour" onClick={onRetour}>
        ← Bibliothèque
      </button>
      <section className="encart">
        <Jaquette jeu={jeu} absent={roms.length === 0} grande />
        <div className="pile serree" style={{ flex: "1 1 300px" }}>
          <div className="sur-titre">
            Génération {jeu.generation} · {console.nom} · {jeu.annee}
          </div>
          <div className="titre-fiche">Pokémon {jeu.nom}</div>
          <div className="attenue">
            {roms.length === 0
              ? "Aucune ROM trouvée pour ce jeu dans votre dossier."
              : `${roms.length} ROM${roms.length > 1 ? "s" : ""} trouvée${roms.length > 1 ? "s" : ""} · émulateur ${console.emulateur === "mgba" ? "mGBA" : "melonDS"}`}
          </div>
        </div>
        <button className="bouton principal" disabled title="Le lancement arrive avec la gestion des émulateurs">
          <IconeJouer />
          Jouer
        </button>
      </section>

      {roms.length > 0 && (
        <section className="pile moyenne">
          <div className="titre-section">
            <h2>ROMs</h2>
          </div>
          <table className="tableau">
            <thead>
              <tr>
                <th>Fichier</th>
                <th>Langue</th>
                <th>Identification</th>
                <th>Taille</th>
                <th>CRC32</th>
                <th>SHA-1</th>
              </tr>
            </thead>
            <tbody>
              {roms.map((r) => (
                <tr key={r.chemin}>
                  <td title={r.chemin}>{r.nomFichier}</td>
                  <td>{r.langue ?? "?"}</td>
                  <td>{methodes[r.methode]}</td>
                  <td>{taille(r.taille)}</td>
                  <td className="mono">{r.crc32}</td>
                  <td className="mono">{r.sha1}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </section>
      )}
    </div>
  );
}
