import { observer } from "mobx-react-lite";
import { rootStore } from "@/stores";
import styles from "./page.module.css";
import { NavLink, Outlet, useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { cn } from "@/utils";

function Page() {
  const { localizations } = rootStore;
  const { t } = useTranslation();
  const navigate = useNavigate();

  if (localizations.isLoading) {
    return (
      <div className={styles.loading}>
        <p>{t("localizations.loading")}</p>
      </div>
    );
  }

  if (localizations.error) {
    return (
      <div className={styles.error}>
        <span>{t("localizations.error")}</span>
        <div className={styles.actions}>
          <button onClick={tryAgain}>{t("localizations.tryAgain")}</button>
          <button onClick={() => navigate("/settings", { replace: true })}>
            {t("localizations.changeSource")}
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      <div className={styles.list}>
        {localizations.all.map((localization) => (
          <NavLink
            key={localization.id}
            to={`/localizations/${localization.id}`}
            className={({ isActive }) =>
              cn(styles.item, isActive && styles.active)
            }
          >
            <img
              src={localizations.flags[localization.id]}
              alt={localization.name}
              className="!w-6 !h-4 shrink-0"
            />
            <span>{localization.name}</span>
          </NavLink>
        ))}
      </div>
      <div className={styles.content}>
        <Outlet />
      </div>
    </div>
  );

  async function tryAgain() {
    await localizations.fetchLocalizations();
  }
}

export default observer(Page);
