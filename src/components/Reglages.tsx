import type { Config, ResultatScan } from "../types";
import { dansTauri } from "../api";

interface Props {
  config: Config;
  scan: ResultatScan | null;
  enCours: boolean;
  onChoisirDossier: () => void;
}

export function Reglages({ config, scan, enCours, onChoisirDossier }: Props) {
  const inconnues = scan?.roms.filter((r) => !r.jeuId) ?? [];
  const reconnues = scan ? scan.roms.length - inconnues.length : 0;

  return (
    <div className="pile">
      <h1 className="titre-fiche">Réglages</h1>

      <section className="panneau pile moyenne">
        <div className="titre-section">
          <h2>Dossier des ROMs</h2>
        </div>
        <div className="attenue">
          Déposez-y les ROMs extraites de vos cartouches (.gb, .gbc, .gba, .nds). Les sous-dossiers sont parcourus. Le lanceur ne modifie
          jamais ces fichiers.
        </div>
        <div className="ligne">
          <code className="chemin">{config.dossierRoms ?? "Aucun dossier choisi"}</code>
          <button className="bouton" onClick={onChoisirDossier} disabled={!dansTauri || enCours}>
            {config.dossierRoms ? "Changer…" : "Choisir…"}
          </button>
        </div>
        {scan && (
          <div className="attenue">
            {enCours ? "Scan en cours…" : `Dernier scan : ${reconnues} ROM${reconnues > 1 ? "s" : ""} reconnue${reconnues > 1 ? "s" : ""}, ${inconnues.length} non reconnue${inconnues.length > 1 ? "s" : ""}, ${scan.ignores.length} fichier${scan.ignores.length > 1 ? "s" : ""} ignoré${scan.ignores.length > 1 ? "s" : ""}.`}
          </div>
        )}
      </section>

      {(inconnues.length > 0 || (scan?.ignores.length ?? 0) > 0) && (
        <section className="panneau pile moyenne">
          <div className="titre-section">
            <h2>Fichiers non reconnus</h2>
          </div>
          <table className="tableau">
            <tbody>
              {inconnues.map((r) => (
                <tr key={r.chemin}>
                  <td title={r.chemin}>{r.nomFichier}</td>
                  <td>{r.entete ? `${r.entete.titre}${r.entete.code ? ` (${r.entete.code})` : ""}` : "entête illisible"}</td>
                  <td className="mono">{r.sha1}</td>
                </tr>
              ))}
              {scan?.ignores.map((f) => (
                <tr key={f.chemin}>
                  <td>{f.chemin}</td>
                  <td colSpan={2}>{f.raison}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </section>
      )}

      <section className="panneau pile moyenne">
        <div className="titre-section">
          <h2>Émulateurs</h2>
        </div>
        <div className="attenue">mGBA (GB / GBC / GBA) et melonDS (DS) seront installés et mis à jour par le lanceur. Prochaine étape.</div>
      </section>
    </div>
  );
}
