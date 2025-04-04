import { makeAutoObservable } from "mobx";
import { StateStore } from "./state";
import { LocalizationsStore } from "./localizations";
import { ActionsStore } from "./actions";

export class RootStore {
  public state: StateStore;
  public localizations: LocalizationsStore;
  public actions: ActionsStore;

  constructor() {
    makeAutoObservable(this);

    this.state = new StateStore();
    this.localizations = new LocalizationsStore();
    this.actions = new ActionsStore();
  }
}

export const rootStore = new RootStore();
