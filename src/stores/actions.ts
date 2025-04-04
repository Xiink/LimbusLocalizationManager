import { makeAutoObservable, runInAction } from "mobx";
import { Localization, Progress, Status } from "./models";
import { invoke } from "@tauri-apps/api/core";
import i18n from "@/i18n";
import { toastError, toastSuccess } from "@/components/toast/toast";
import { computedFn } from "mobx-utils";
import { listen } from "@tauri-apps/api/event";

export class ActionsStore {
  public status: Record<string, Status> = {};
  public startingGame: boolean = false;
  public progressLog: Progress[] = [];

  constructor() {
    makeAutoObservable(this, {}, { autoBind: true });

    listen("play:started", () =>
      runInAction(() => {
        this.progressLog.push({ type: "started" });
      })
    );

    listen<string>("play:unknown_localization", (event) =>
      runInAction(() => {
        this.progressLog.push({
          type: "unknown_localization",
          localization: event.payload,
        });
      })
    );

    listen<string>("play:up_to_date", (event) =>
      runInAction(() => {
        this.progressLog.push({
          type: "up_to_date",
          localization: event.payload,
        });
      })
    );

    listen<string>("play:updating", (event) =>
      runInAction(() => {
        this.progressLog.push({
          type: "updating",
          localization: event.payload,
        });
      })
    );

    listen<string>("play:update_finished", (event) =>
      runInAction(() => {
        this.progressLog.push({
          type: "update_finished",
          localization: event.payload,
        });
      })
    );

    listen("play:starting_game", () =>
      runInAction(() => {
        this.progressLog.push({ type: "starting_game" });
      })
    );

    listen("play:finished", () =>
      runInAction(() => {
        this.progressLog.push({ type: "finished" });
      })
    );
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

  public async updateAndPlay() {
    if (this.startingGame) {
      throw new Error("Game is already starting");
    }

    this.startingGame = true;
    this.progressLog = [];

    try {
      await invoke("update_and_play");
    } catch (error) {
      console.error(error);
      toastError(i18n.t("error.updateAndPlay"));
    } finally {
      runInAction(() => {
        this.startingGame = false;
      });
    }
  }

  public getStatus = computedFn((localization: Localization) => {
    return this.status[localization.id] ?? Status.Idle;
  });
}
