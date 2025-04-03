import { invoke } from "@tauri-apps/api/core";
import styles from "./page.module.css";
import { useTranslation } from "react-i18next";

function Page() {
  const { t } = useTranslation();

  return (
    <div className={styles.container}>
      <button className={styles.play} onClick={playGame}>
        {t("home.play")}
      </button>
    </div>
  );

  async function playGame() {
    await invoke("update_and_play");
  }
}

export default Page;
