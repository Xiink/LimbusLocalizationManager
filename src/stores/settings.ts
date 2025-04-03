import { makeAutoObservable, reaction, runInAction } from "mobx";
import { AppSettings } from "./models";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import i18n, { languageNames } from "@/i18n";

export class SettingsStore {
  public settings: AppSettings | null = null;
  public isLoading: boolean = false;
  public isSaving: boolean = false;

  constructor() {
    makeAutoObservable(this, {}, { autoBind: true });

    listen<AppSettings>("settings_updated", (event) => {
      runInAction(() => {
        this.settings = event.payload;
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
    )
  }

  async loadSettings() {
    if (this.isLoading) {
      return;
    }

    this.isLoading = true;

    try {
      const settings = await invoke<AppSettings>("get_settings");
      runInAction(() => {
        this.settings = settings;
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

  public get language() {
    return (
      this.settings?.language ?? i18n.language
    ) as keyof typeof languageNames;
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
    return this.settings?.installed;
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
    return this.settings !== null;
  }
}
