import styles from "./page.module.css";
import { useTranslation } from "react-i18next";
import Console from "./console";
import { observer } from "mobx-react-lite";
import { rootStore } from "@/stores";
import { cn } from "@/utils";
import { Languages } from "lucide-react";
import { NavLink } from "react-router";

function Page() {
  const { t } = useTranslation();
  const { actions } = rootStore;

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <span className={styles.version}>
          v{import.meta.env.VITE_APP_VERSION || "1.0.0"}
          {" "}
          <a href={import.meta.env.VITE_APP_SOURCE_URL} target="_blank" rel="noopener noreferrer">
            {t("home.updateAvailable")}
          </a>
        </span>

        <NavLink to="/settings#interface-language" className={styles.language}>
          <Languages />
        </NavLink>
      </div>

      <div className={styles.main}>
        <span className={styles.title}>
          {t("home.title")}
        </span>
      </div>

      <div className={styles.actions}>
        <button 
          className={cn(styles.play, actions.startingGame && styles.loading)} 
          onClick={playGame} 
          disabled={actions.startingGame}
        >
          {t("home.play")}
        </button>
        <Console />
      </div>
    </div>
  );

  async function playGame() {
    await actions.updateAndPlay();
  }
}

export default observer(Page);
