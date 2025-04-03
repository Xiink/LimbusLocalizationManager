import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import LanguageDetector from "i18next-browser-languagedetector";

import en from "./locales/en.json";
import ru from "./locales/ru.json";

const resources = {
  en: {
    translation: en,
  },
  ru: {
    translation: ru,
  },
};

export const languageNames = {
  en: "English",
  ru: "Русский",
} as const;

i18n
  .use(initReactI18next)
  .use(LanguageDetector)
  .init({
    lng: "en",
    fallbackLng: "en",
    resources,
  });

export default i18n;
