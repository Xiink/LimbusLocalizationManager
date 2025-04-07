# Software Internationalization Guide (for Translators)

## Preparation Steps

1. **Locate the translation folder**
   - Open the project in your file explorer
   - Navigate to: `src/locales/`
   - Find the base file `en.json`

2. **Create your language template**
   - Right-click `en.json` → "Copy"
   - Right-click empty space → "Paste"
      ![en - Copy.json](https://github.com/user-attachments/assets/22720149-1f81-468b-b02d-89960a58f3a9)
   - Rename the copy using proper format: (If you are not sure about the **Format**, you can check it in [BCP47 Locale Name](https://ss64.com/locale.html))

      ```txt
      Format: [language]-[REGION].json
      Example: es-ES.json (Spanish/Spain)
               fr-CA.json (French/Canada)
               zh-Hant.json (Chinese Traditional)
      ```

      ![Rename](https://github.com/user-attachments/assets/0e480fba-5e7f-4b77-99ed-b6a17910679f)

## Translation Process

1. **Edit the translation file**
   - Open the new file with any text editor
   - **Only modify** the text **after** the colon (`:`) on each line
   - Keep the **quoted text before colon** unchanged
      ![Key-Value pair](https://github.com/user-attachments/assets/3dd36bac-145a-4d93-876f-3cb259771ce7)
   - Example:

      ```jsonc
      // BEFORE
      "title": "Limbus Localization Manager",
      
      // AFTER (Chinese example)
      "title": "邊獄巴士翻譯管理器",
      ```

2. **Special cases handling**
   - Preserve `{{variable}}` placeholders exactly as found
   - Example:

      ```jsonc
      // Original
      "installed": "Installed {{localization}} ({{version}})",
      
      // Correct translation
      "installed": "已安裝 {{localization}}（{{version}}）",
      
      // Incorrect translation (modified placeholder)
      "installed": "已安裝 {localization}（{version}）",
      "installed": "已安裝 {{本地化}}（{{版本}}）",
      ```

## Connecting Translations to Software

### **Register your translation**

- Navigate to `src/i18n.ts`
   ![i18n.ts](https://github.com/user-attachments/assets/932033c7-4f6f-46de-87ca-23755295a1b7)
- Find these three code sections:

   ```ts
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
   ```

### Section 1: File Imports (Add New Line)

```ts
// Part I: Import
import en from "./locales/en.json";
import ru_RU from "./locales/ru-RU.json";
// Add your line here following this pattern:
import [YOUR_CODE] from "./locales/[YOUR_FILE].json";
```

- Replace `[YOUR_FILE]` with your filename from **Step 2** at **Preparation Steps**
- Replace `[YOUR_CODE]` using this conversion:
  - Change hyphens `-` to underscores `_`
  - Example: `zh-Hant.json` → `zh_Hant`

### Section 2: Translation Resources (Add New Entry)

```ts
// Part II: Resources
const resources = {
  en: { translation: en },
  ru_RU: { translation: ru_RU },
  // Add your entry here following this pattern:
  [YOUR_CODE]: { translation: [YOUR_IMPORT] },
};
```

- Use the **same code** from **Section 1**
- Example for Chinese Traditional:

  ```ts
  zh_Hant: { translation: zh_Hant },
  ```

### Section 3: Language Display Names (Add New Entry)

```ts
// Part III: Language Names
export const languageNames = {
  en: "English",
  ru_RU: "Русский",
  // Add your entry here:
  [YOUR_CODE]: "[NATIVE_LANGUAGE_NAME]",
} as const;
```

- Use the **same code** from previous sections
- Write the language name in its native form:
  - Spanish: `"Español"`
  - French: `"Français"`
  - Chinese Traditional: `"繁體中文"`
- Example for Chinese Traditional:

  ```ts
  zh_Hant: "繁體中文",
  ```

## Quality Checklist

Before submitting:

- [ ] No modified text before colons (`:`) in JSON
- [ ] All `{{variables}}` preserved exactly
- [ ] File named correctly (hyphens, no spaces)
- [ ] Native name uses correct diacritics (é, ñ, ç)
- [ ] Tested in software (if possible)
