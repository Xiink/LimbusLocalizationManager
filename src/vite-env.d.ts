/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_APP_SOURCE_URL: string;
  readonly VITE_APP_RELEASE_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
