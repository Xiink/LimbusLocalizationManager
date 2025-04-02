export interface LocalizationSource {
  name: string;
  url: string;
}

export interface AppSettings {
  installed: Record<string, Localization>;
  sources: Record<string, LocalizationSource>;
  selected_source: string | null;
  game_directory: string | null;
  language: string | null;
}

export interface AvailableLocalizations {
  localizations: Localization[];
}

export const Format = {
  Compatible: "compatible",
  New: "new"
} as const;

export type Format = (typeof Format)[keyof typeof Format];

export interface Localization {
  id: string;
  version: string;
  name: string;
  country: string;
  icon: string;
  description: string;
  authors: string[];
  urls: string[];
  format: Format;
}

export const Status = {
  Idle: "idle",
  Installing: "installing",
  Uninstalling: "uninstalling",
  Updating: "updating",
  Repairing: "repairing",
} as const;

export type Status = (typeof Status)[keyof typeof Status];
