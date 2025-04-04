import { observer } from "mobx-react-lite";
import styles from "./page.module.css";
import { ArrowDown, Folder, X } from "lucide-react";
import { useTranslation } from "react-i18next";
import { rootStore } from "@/stores";
import { open } from "@tauri-apps/plugin-dialog";
import { toastError } from "@/components/toast/toast";
import { languageNames } from "@/i18n";
import { useLocation } from "react-router";
import { cn } from "@/utils";

function Page() {
  const { t } = useTranslation();
  const { settings } = rootStore;

  const { hash } = useLocation();

  return (
    <div className={styles.container}>
      <h1 className="text-xl">{t("settings.title")}</h1>

      <div className={styles.settings}>
        <div className={cn(styles.section, hash === "#interface-language" && styles.active)}>
          <h2>
            {t("settings.interfaceLanguage")}
          </h2>
          <div className={styles.select}>
            <select
              value={settings.language}
              onChange={handleLanguageChange}
            >
              {Object.entries(languageNames).map(([key, name]) => (
                <option key={key} value={key}>
                  {name}
                </option>
              ))}
            </select>
            <ArrowDown strokeWidth={1.5} />
          </div>
        </div>

        <div className={styles.section}>
          <h2>{t("settings.source")}</h2>
          <div className={styles.select}>
            <select
              value={settings.selectedSource ?? undefined}
              onChange={handleSourceChange}
            >
              {Object.entries(settings.sources!).map(([key, source]) => (
                <option key={key} value={key}>
                  {source.name}
                </option>
              ))}
            </select>
            <ArrowDown strokeWidth={1.5} />
          </div>
        </div>

        <div className={styles.section}>
          <h2>{t("settings.gameDirectory")}</h2>
          <div className="flex gap-2 items-center">
            <input
              type="text"
              className={styles.input}
              value={settings.gameDirectory ?? ""}
              placeholder={t("settings.gameDirectoryDefault")}
              disabled
            />
            {settings.gameDirectory === null ? (
              <button className={styles.button} onClick={handleGameDirectoryPick}>
                <Folder />
              </button>
            ) : (
              <button className={styles.button} onClick={handleGameDirectoryClear}>
                <X />
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );

  function handleSourceChange(event: React.ChangeEvent<HTMLSelectElement>) {
    settings.selectSource(event.target.value);
  }

  async function handleGameDirectoryPick() {
    const directory = await open({
      directory: true,
      multiple: false,
    });

    if (!directory) {
      return;
    }

    try {
      await settings.setGameDirectory(directory);
    } catch (error) {
      console.error(error);
      toastError(t("error.setGameDirectory"));
    }
  }

  async function handleGameDirectoryClear() {
    try {
      await settings.setGameDirectory(null);
    } catch (error) {
      console.error(error);
      toastError(t("error.setGameDirectory"));
    }
  }

  function handleLanguageChange(event: React.ChangeEvent<HTMLSelectElement>) {
    settings.setLanguage(
      event.target.value as keyof typeof languageNames
    );
  }
}

export default observer(Page);
