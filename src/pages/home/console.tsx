import { rootStore } from "@/stores";
import { useEffect } from "react";
import { useRef } from "react";
import { observer } from "mobx-react-lite";
import styles from "./console.module.css";
import { useTranslation } from "react-i18next";
import Log from "./log";

function Console() {
  const { actions } = rootStore;
  const { t } = useTranslation();
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!ref.current) {
      return;
    }

    ref.current.scrollTo({
      top: ref.current.scrollHeight,
      behavior: 'smooth'
    });
  }, [actions.progressLog.length, ref.current]);

  return (
    <div className={styles.console}>
      <span className={styles.title}>
        {t("home.console")}
      </span>
      <div className={styles.container} ref={ref}>
        {actions.progressLog.map((log, index) => (
          <Log key={index} progress={log} />
        ))}
      </div>
    </div>
  );
}

export default observer(Console);
