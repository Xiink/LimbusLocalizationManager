import { getCurrentWindow } from "@tauri-apps/api/window";
import { useMemo } from "react";
import { X, Minus } from "lucide-react";
import styles from "./styles.module.css";
import { useTranslation } from "react-i18next";
function Header() {
  const { t } = useTranslation();
  const appWindow = useMemo(() => getCurrentWindow(), []);

  return (
    <header 
      data-tauri-drag-region="true"
      className={styles.header}
    >
      <span>{t("title")}</span>
      <div className={styles.controls}>
        <button onClick={handleMinimize}>
          <Minus className="w-4 h-4" />
        </button>
        <button onClick={handleClose}>
          <X className="w-4 h-4" />
        </button>
      </div>
    </header>
  );

  async function handleClose() {
    await appWindow.close();
  }

  async function handleMinimize() {
    await appWindow.minimize();
  }
}

export default Header;
