import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import { locale } from "@tauri-apps/plugin-os";
import type { LanguageDetectorAsyncModule } from "i18next";

// Part I: Importing Language Files
import en from "./locales/en.json";
import ru_RU from "./locales/ru-RU.json";
import zh_Hant from "./locales/zh-Hant.json";

// Part II: Resource definition
const resources = {
  en: { translation: en },
  ru_RU: { translation: ru_RU },
  zh_Hant: { translation: zh_Hant },
};

// Part III: Language display name
export const languageNames = {
  en: "English",
  ru_RU: "Русский",
  zh_Hant: "繁體中文",
} as const;

const languageDetector: LanguageDetectorAsyncModule = {
  type: "languageDetector",
  async: true,
  detect: (callback) => {
    locale()
      .then((language) => {
        callback(language ?? "en");
      })
      .catch(() => callback("en"));
  },
};

i18n
  .use(initReactI18next)
  .use(languageDetector)
  .init({
    fallbackLng: {
      "zh-TW": ["zh_Hant", "en"],
      "zh-HK": ["zh_Hant", "en"],
      "ru-RU": ["ru_RU", "en"],
      "ru-*": ["ru_RU", "en"],
      default: ["en"],
    },
    resources,
    nonExplicitSupportedLngs: true
  });

export default i18n;
