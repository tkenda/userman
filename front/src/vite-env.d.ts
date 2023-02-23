/// <reference types="vite/client" />
/// <reference path="./../entities.d.ts" />
declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

declare module "vue-json-viewer";
