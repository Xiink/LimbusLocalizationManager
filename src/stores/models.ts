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
  New: "new",
} as const;

export type Format = (typeof Format)[keyof typeof Format];

export interface Font {
  url: string;
  hash: string;
}

export interface Localization {
  id: string;
  version: string;
  name: string;
  flag: string;
  icon: string;
  description: string;
  authors: string[];
  url: string;
  font: Font;
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
