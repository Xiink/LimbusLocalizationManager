import { rootStore } from "@/stores";
import { observer } from "mobx-react-lite";
import { useParams } from "react-router";
import styles from "./page.module.css";
import Markdown from "@/components/markdown/markdown";
import Actions from "./actions";
import { ArrowLeft } from "lucide-react";
import { useTranslation } from "react-i18next";

function Page() {
  const { id } = useParams();
  const { t } = useTranslation();
  const { localizations } = rootStore;

  if (!id) {
    return (
      <div className="h-full flex gap-2 items-center justify-center text-limbus-500">
        <ArrowLeft className="w-6 h-6" strokeWidth={1.5} />
        <h1>{t("localization.select")}</h1>
      </div>
    );
  }

  const localization = localizations.byId[id];

  if (!localization) {
    return (
      <div className="h-full flex gap-2 items-center justify-center text-danger">
        <h1>{t("localization.notFound")}</h1>
      </div>
    );
  }

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <img src={localization.icon} alt={localization.name} />
        <div className={styles.info}>
          <h1>{localization.name}</h1>
          <span>
            {t("localization.authors")}: {localization.authors.join(", ")}
          </span>
        </div>
        <Actions localization={localization} />
      </div>
      <div className={styles.description}>
        <Markdown>{localization.description}</Markdown>
      </div>
    </div>
  );
}

export default observer(Page);
