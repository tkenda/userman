<template>
  <v-app-bar color="primary" density="compact">
    <template v-slot:prepend>
      <v-app-bar-nav-icon @click="toggleNavbar" />
    </template>

    <v-app-bar-title class="d-none d-md-flex">{{ title }}</v-app-bar-title>

    <div class="search-text">
      <v-text-field
        v-model="search"
        hide-details
        prepend-icon="mdi-magnify"
        single-line
      ></v-text-field>
    </div>
  </v-app-bar>
</template>

<script lang="ts">
import { useAppStore } from "@/store/app";

export default {
  name: "AppBar",
  emit: ["toggleNavbar"],
  data: () => ({
    title: "Userman",
    search: "",
  }),
  watch: {
    search: {
      handler(newValue: string, _oldValue) {
        const app = useAppStore();
        app.setSearchText(newValue);
      },
    },
  },
  methods: {
    toggleNavbar: function () {
      this.$emit("toggleNavbar");
    },
  },
};
</script>

<style scoped>
.search-text {
  width: 400px;
}
</style>
