import { makeAutoObservable, reaction, runInAction } from "mobx";
import { AppState } from "./models";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import i18n, { languageNames } from "@/i18n";

export class StateStore {
  public state: AppState | null = null;
  public isLoading: boolean = false;
  public isSaving: boolean = false;
  public latestVersion: string | null = null;

  constructor() {
    makeAutoObservable(this, {}, { autoBind: true });

    listen<AppState>("app_state_updated", (event) => {
      runInAction(() => {
        this.state = event.payload;
      });
    });

    reaction(
      () => this.settings?.language,
      () => {
        if (this.settings?.language) {
          console.log("changeLanguage", this.settings.language);
          i18n.changeLanguage(this.settings.language);
        }
      }
    );
  }

  async loadLatestVersion() {
    const version = await invoke<string>("get_latest_version");
    runInAction(() => {
      this.latestVersion = version;
    });
  }

  async loadState() {
    if (this.isLoading) {
      return;
    }

    this.isLoading = true;

    try {
      const state = await invoke<AppState>("get_app_state");
      runInAction(() => {
        this.state = state;
      });
    } catch (error) {
      console.error(error);
      throw error;
    } finally {
      runInAction(() => {
        this.isLoading = false;
      });
    }
  }

  async saveSettings() {
    if (!this.settings) {
      throw new Error("Settings are not loaded");
    }

    if (this.isSaving) {
      return;
    }

    this.isSaving = true;

    try {
      await invoke("update_settings", { newSettings: this.settings });
    } catch (error) {
      console.error(error);
    } finally {
      runInAction(() => {
        this.isSaving = false;
      });
    }
  }

  public get isUpdateAvailable() {
    return (
      this.latestVersion !== null &&
      this.latestVersion !== import.meta.env.VITE_APP_VERSION
    );
  }

  public get settings() {
    return this.state?.settings;
  }

  public get language() {
    return (this.settings?.language ??
      i18n.language) as keyof typeof languageNames;
  }

  public setLanguage(language: keyof typeof languageNames) {
    if (!this.settings) {
      throw new Error("Settings are not loaded");
    }

    this.settings.language = language;
    this.saveSettings();
  }

  public get sources() {
    return this.settings?.sources;
  }

  public selectSource(source: string) {
    if (!this.settings) {
      throw new Error("Settings are not loaded");
    }

    this.settings.selected_source = source;
    this.saveSettings();
  }

  public get selectedSource() {
    return this.settings?.selected_source;
  }

  public get installed() {
    return this.state?.installed_metadata?.installed ?? {};
  }

  public get hasInstalledLocalizations() {
    return Object.keys(this.installed).length > 0;
  }

  public get gameDirectory() {
    return this.settings?.game_directory ?? null;
  }

  public async setGameDirectory(directory: string | null) {
    if (!this.settings) {
      throw new Error("Settings are not loaded");
    }

    await invoke("set_game_directory", { directory });
  }

  public get isReady() {
    return this.state !== null;
  }
}
