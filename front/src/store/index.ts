// Utilities
import { createPinia } from "pinia";

import router from "../router";

import { markRaw } from "vue";

import type { Router } from "vue-router";

declare module "pinia" {
  export interface PiniaCustomProperties {
    router: Router;
  }
}

const pinia = createPinia();

pinia.use(({ store }) => {
  store.router = markRaw(router);
});

export default pinia;
