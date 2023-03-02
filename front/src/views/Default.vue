<template>
  <v-layout>
    <div v-if="error" class="sticky">
      <v-alert :text="error" type="error" closable></v-alert>
    </div>
    <div v-if="info" class="sticky">
      <v-alert :text="info" type="info" closable></v-alert>
    </div>
    <app-bar @toggleNavbar="toggleNavbar = !toggleNavbar" />
    <nav-bar :toggleNavbar="toggleNavbar" />
    <v-main>
      <router-view />
    </v-main>
  </v-layout>
</template>

<script lang="ts">
import { storeToRefs } from "pinia";
import { useAppStore } from "@/store/app";

import AppBar from "../components/AppBar.vue";
import NavBar from "../components/NavBar.vue";

export default {
  name: "Default",
  components: {
    AppBar,
    NavBar,
  },
  setup() {
    const app = useAppStore();
    const { getErrorMessage, getInfoMessage } = storeToRefs(app);
    return { getErrorMessage, getInfoMessage };
  },
  data: () => ({
    toggleNavbar: false,
    error: "",
    info: "",
  }),
  watch: {
    getErrorMessage(newValue, _oldValue) {
      this.error = newValue;
    },
    getInfoMessage(newValue, _oldValue) {
      this.info = newValue;
    },
  },
};
</script>

<style scoped>
.sticky {
  position: absolute;
  z-index: 999999;
  top: 20px;
  left: 50%;
  transform: translate(-50%, 0%);
}
</style>
