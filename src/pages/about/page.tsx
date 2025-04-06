import { useTranslation } from "react-i18next";
import styles from "./page.module.css";
import Kim from "./kim";

function Page() {
  const { t } = useTranslation();

  return (
    <div className={styles.container}>
      <h1 className="text-xl">{t("about.title")}</h1>
      <div className={styles.section}>
        <h2 className="text-lg">{t("about.credits")}</h2>
        <ul className={styles.list}>
          <li>kimght - {t("about.developer")}</li>
        </ul>
      </div>
      <div className={styles.section}>
        <h2 className="text-lg">{t("about.links")}</h2>
        <ul className={styles.list}>
          <li>
            <a
              target="_blank"
              rel="noopener noreferrer"
              href={import.meta.env.VITE_APP_REPO_URL}
              className="lnk"
            >
              {t("about.source")}
            </a>
          </li>
          <li>
            <a
              target="_blank"
              rel="noopener noreferrer"
              href={`${import.meta.env.VITE_APP_REPO_URL}/issues`}
              className="lnk"
            >
              {t("about.issues")}
            </a>
          </li>
        </ul>
      </div>

      <Kim />

      <span className="absolute bottom-1 right-1 text-xs text-limbus-600 select-none italic">
        {`/pmg/`}
      </span>
    </div>
  );
}

export default Page;
