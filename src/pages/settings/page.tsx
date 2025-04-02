import { observer } from "mobx-react-lite";
import styles from "./page.module.css";
import { ArrowDown } from "lucide-react";
import { useTranslation } from "react-i18next";
import { rootStore } from "@/stores";

function Page() {
  const { t } = useTranslation();
  const { settings } = rootStore;

  return (
    <div className={styles.container}>
      <h1 className="text-xl">{t("settings.title")}</h1>

      <h2>{t("settings.interfaceLanguage")}</h2>
      <div className={styles.select}>
        <select>
          <option value="en">English</option>
          <option value="pl">Polish</option>
          <option value="de">German</option>
          <option value="es">Spanish</option>
          <option value="fr">French</option>
          <option value="it">Italian</option>
        </select>
        <ArrowDown strokeWidth={1.5}/>
      </div>

      <h2>{t("settings.source")}</h2>
      <div className={styles.select}>
        <select value={settings.selectedSource ?? undefined} onChange={handleSourceChange}>
          {Object.entries(settings.sources!).map(([key, source]) => (
            <option key={key} value={key}>{source.name}</option>
          ))}
        </select>
        <ArrowDown strokeWidth={1.5}/>
      </div>
    </div>
  );

  function handleSourceChange(event: React.ChangeEvent<HTMLSelectElement>) {
    settings.selectSource(event.target.value);
  }
}

export default observer(Page);
