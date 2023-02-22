import { App } from "vue";
import VueAxios from "vue-axios";

import { loadFonts } from "./webfontloader";

import axios from "./axios";
import vuetify from "./vuetify";

import router from "../router";
import pinia from "../store";

export function registerPlugins(app: App) {
  loadFonts();

  app.use(vuetify);
  app.use(router);
  app.use(pinia);
  app.use(VueAxios, axios);
}
