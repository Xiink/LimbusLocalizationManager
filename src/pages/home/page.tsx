import styles from "./page.module.css";
import { useTranslation } from "react-i18next";
import Console from "./console";
import { observer } from "mobx-react-lite";
import { rootStore } from "@/stores";
import { cn } from "@/utils";
import { Languages } from "lucide-react";
import { NavLink, useNavigate } from "react-router";

function Page() {
  const { t } = useTranslation();
  const { actions, state } = rootStore;
  const navigate = useNavigate();

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        {state.currentVersion && (
          <span className={styles.version}>
            v{state.currentVersion}{" "}
            {state.isUpdateAvailable && (
              <a
                href={`${import.meta.env.VITE_APP_REPO_URL}/releases/latest`}
                target="_blank"
                rel="noopener noreferrer"
              >
                {t("home.updateAvailable")}
              </a>
            )}
          </span>
        )}

        <NavLink to="/settings#interface-language" className={styles.language}>
          <Languages />
        </NavLink>
      </div>

      <div className={styles.main}>
        <img src="/star.png" alt="logo" className={styles.logo} />

        <span className={styles.title}>{t("home.title")}</span>

        {!state.hasInstalledLocalizations && (
          <span className={styles.subtitle}>
            {t("home.noLocalizationsInstalled")}
          </span>
        )}
      </div>

      <div className={styles.actions}>
        <button
          className={cn(styles.play, actions.startingGame && styles.loading)}
          onClick={handleClick}
          disabled={actions.startingGame}
        >
          {state.hasInstalledLocalizations ? t("home.play") : t("home.add")}
        </button>
        <Console />
      </div>
    </div>
  );

  async function handleClick() {
    if (!state.hasInstalledLocalizations) {
      navigate("/localizations");
      return;
    }

    await actions.updateAndPlay();
  }
}

export default observer(Page);
