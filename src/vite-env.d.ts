/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_APP_REPO_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
