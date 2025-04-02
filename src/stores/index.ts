import { makeAutoObservable } from "mobx";
import { SettingsStore } from "./settings";
import { LocalizationsStore } from "./localizations";
import { ActionsStore } from "./actions";

export class RootStore {
  public settings: SettingsStore;
  public localizations: LocalizationsStore;
  public actions: ActionsStore;

  constructor() {
    makeAutoObservable(this);

    this.settings = new SettingsStore();
    this.localizations = new LocalizationsStore();
    this.actions = new ActionsStore();
  }
}

export const rootStore = new RootStore();
