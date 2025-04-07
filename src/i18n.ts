import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import { locale } from "@tauri-apps/plugin-os";
import type { LanguageDetectorAsyncModule } from "i18next";
import en from "./locales/en.json";
import ru from "./locales/ru.json";

const resources = {
  en: {
    translation: en,
  },
  ru: {
    translation: ru,
  }
};

export const languageNames = {
  en: "English",
  ru: "Русский",
} as const;

const languageDetector: LanguageDetectorAsyncModule = {
  type: "languageDetector",
  async: true,
  detect: (callback: (lng: string | readonly string[] | undefined) => void) => {
    locale()
      .then((language) => {
        console.log(`Detected language: ${language}`);
        callback(language ?? "en");
      })
      .catch((error) => {
        console.error("Error detecting language:", error);
        callback("en");
      });
  },
};

i18n
  .use(initReactI18next)
  .use(languageDetector)
  .init({
    fallbackLng: {
      "zh-TW": ["zh-Hant", "en"],
      default: ["en"],
    },
    resources,
  });

export default i18n;
