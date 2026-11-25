import catalogue from "../../data/games.json";
import type { Jeu } from "../types";

/** Jaquette en aplat de couleur (pas d'image officielle). */
export function Jaquette({ jeu, absent = false, grande = false }: { jeu: Jeu; absent?: boolean; grande?: boolean }) {
  const console = catalogue.consoles[jeu.console as keyof typeof catalogue.consoles];
  return (
    <div
      className={grande ? "jaquette grande" : "jaquette"}
      style={{ background: jeu.couleur, color: jeu.encre, opacity: absent ? 0.35 : 1 }}
    >
      <span className="badge-console">{console.court}</span>
      <div>
        <div className="pokemon">Pokémon</div>
        <div className="nom-jeu">{jeu.nom}</div>
      </div>
    </div>
  );
}
