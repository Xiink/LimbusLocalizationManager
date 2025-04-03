import styles from "./page.module.css";
import { useTranslation } from "react-i18next";

function Page() {
  const { t } = useTranslation();

  return (
    <div className={styles.container}>
      <button className={styles.play}>
        {t("home.play")}
      </button>
    </div>
  );
}

export default Page;
