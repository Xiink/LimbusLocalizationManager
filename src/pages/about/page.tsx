import { useTranslation } from "react-i18next";
import styles from "./page.module.css";

function Page() {
  const { t } = useTranslation();

  return (
    <div className={styles.container}>
      <h1 className="text-xl">{t("about.title")}</h1>
    </div>
  );
}

export default Page;
