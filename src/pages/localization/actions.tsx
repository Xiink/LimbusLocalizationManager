import { rootStore } from "@/stores";
import { Localization, Status } from "@/stores/models";
import { observer } from "mobx-react-lite";
import styles from "./actions.module.css";
import { Hammer, Loader2, Plus, X } from "lucide-react";
import { useTranslation } from "react-i18next";

interface ActionsProps {
  localization: Localization;
}

function Actions({ localization }: ActionsProps) {
  const { settings, actions } = rootStore;
  const { t } = useTranslation();

  const installedVersion = settings.installed?.[localization.id]?.version;
  const status = actions.getStatus(localization);

  return (
    <div className={styles.container}>
      <div className={styles.controls}>
        {status === Status.Idle && (
          <>
            {!installedVersion && (
              <button onClick={handleInstall} title={t("localization.add")}>
                <Plus className="w-6 h-6 shrink-0" />
              </button>
            )}

            {installedVersion && (
              <>
                <button onClick={handleRepair} title={t("localization.repair")}>
                  <Hammer className="w-6 h-6 shrink-0" />
                </button>
                <button
                  onClick={handleUninstall}
                  title={t("localization.uninstall")}
                >
                  <X className="w-6 h-6 shrink-0" />
                </button>
              </>
            )}
          </>
        )}

        {status === Status.Installing && (
          <div className={styles.installing}>
            <Loader2 className="w-6 h-6 shrink-0 animate-spin text-limbus-500" />
          </div>
        )}
      </div>

      {installedVersion && installedVersion !== localization.version && (
        <div className={styles.updates}>
          {installedVersion} → {localization.version}
        </div>
      )}
    </div>
  );

  function handleInstall() {
    actions.install(localization);
  }

  function handleUninstall() {
    actions.uninstall(localization);
  }

  function handleRepair() {
    actions.repair(localization);
  }
}

export default observer(Actions);
