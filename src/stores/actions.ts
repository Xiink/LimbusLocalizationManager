import { makeAutoObservable, runInAction } from "mobx";
import { Localization, Status } from "./models";
import { invoke } from "@tauri-apps/api/core";
import i18n from "@/i18n";
import { toastError, toastSuccess } from "@/components/toast/toast";
import { computedFn } from "mobx-utils";

export class ActionsStore {
  public status: Record<string, Status> = {};

  constructor() {
    makeAutoObservable(this, {}, { autoBind: true });
  }

  public async install(localization: Localization) {
    const status = this.status[localization.id] ?? Status.Idle;

    if (status !== Status.Idle) {
      throw new Error("Has operation in progress");
    }

    this.status[localization.id] = Status.Installing;

    try {
      await invoke("install_localization", { localization });
      toastSuccess(
        i18n.t("localization.installed", {
          localization: localization.name,
          version: localization.version,
        })
      );
    } catch (error) {
      toastError(i18n.t("error.install", { localization: localization.name }));
      console.error(error);
    } finally {
      runInAction(() => {
        this.status[localization.id] = Status.Idle;
      });
    }
  }

  public async uninstall(localization: Localization) {
    const status = this.status[localization.id] ?? Status.Idle;

    if (status !== Status.Idle) {
      throw new Error("Has operation in progress");
    }

    this.status[localization.id] = Status.Uninstalling;

    try {
      await invoke("uninstall_localization", { localization });
      toastSuccess(
        i18n.t("localization.uninstalled", { localization: localization.name })
      );
    } catch (error) {
      toastError(i18n.t("error.uninstall"));
      console.error(error);
    } finally {
      runInAction(() => {
        this.status[localization.id] = Status.Idle;
      });
    }
  }

  public async repair(localization: Localization) {
    const status = this.status[localization.id] ?? Status.Idle;

    if (status !== Status.Idle) {
      throw new Error("Has operation in progress");
    }

    this.status[localization.id] = Status.Repairing;

    try {
      await invoke("repair_localization", { localization });
      toastSuccess(
        i18n.t("localization.repaired", { localization: localization.name })
      );
    } catch (error) {
      toastError(i18n.t("error.repair"));
      console.error(error);
    } finally {
      runInAction(() => {
        this.status[localization.id] = Status.Idle;
      });
    }
  }

  public getStatus = computedFn((localization: Localization) => {
    return this.status[localization.id] ?? Status.Idle;
  });
}
