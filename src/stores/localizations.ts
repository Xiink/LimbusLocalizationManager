import { makeAutoObservable, runInAction } from "mobx";
import { Localization } from "./models";
import { invoke } from "@tauri-apps/api/core";

const getFlag = async (localization: Localization) => {
  try {
    const flag = await import(`../assets/flags/${localization.flag}.svg`);
    return { flag: flag.default, id: localization.id };
  } catch (error) {
    return {
      flag: `https://purecatamphetamine.github.io/country-flag-icons/3x2/${localization.flag}.svg`,
      id: localization.id,
    };
  }
};

export class LocalizationsStore {
  public byId: Record<string, Localization> = {};
  public flags: Record<string, string> = {};
  public isLoading: boolean = false;
  public error: string | null = null;

  constructor() {
    makeAutoObservable(this, {}, { autoBind: true });

    this.fetchLocalizations();
  }

  async fetchLocalizations() {
    this.isLoading = true;
    this.error = null;

    try {
      const localizations = await invoke<Localization[]>(
        "get_available_localizations"
      );

      const flags = await Promise.all(localizations.map(getFlag));

      runInAction(() => {
        this.byId = localizations.reduce(
          (acc, localization) => {
            acc[localization.id] = localization;
            return acc;
          },
          {} as Record<string, Localization>
        );

        this.flags = flags.reduce(
          (acc, { flag, id }) => {
            acc[id] = flag;
            return acc;
          },
          {} as Record<string, string>
        );
      });
    } catch (error) {
      console.error(error);
      runInAction(() => {
        this.error = error as string;
      });
    } finally {
      runInAction(() => {
        this.isLoading = false;
      });
    }

    
  }

  public get all() {
    return Object.values(this.byId).sort((a, b) =>
      a.name.localeCompare(b.name)
    );
  }
}
