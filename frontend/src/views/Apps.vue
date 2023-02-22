<template>
  <template v-if="compactMode">
    <v-row no-gutters>
      <v-col>
        <v-btn class="mt-2 mx-2" color="primary" @click="openNew">
          New
          <v-icon end icon="mdi-plus"></v-icon>
        </v-btn>
        <apps-list
          :apps="apps"
          :unselect="unselect"
          @selected="handleSelection"
        />
      </v-col>
    </v-row>
    <v-dialog v-model="dialog">
      <app-card
        :app="app"
        @updated="handleUpdated"
        @created="handleCreatedDeleted"
        @deleted="handleCreatedDeleted"
        @unselect="handleUnselect"
        @close="handleClose"
        closeButton
      />
    </v-dialog>
  </template>
  <template v-else>
    <v-row no-gutters>
      <v-col style="padding: 20px 0px 20px 20px">
        <apps-list
          :apps="apps"
          :unselect="unselect"
          @selected="handleSelection"
          removePadding
        />
      </v-col>
      <v-col style="padding: 20px">
        <app-card
          :app="app"
          @updated="handleUpdated"
          @created="handleCreatedDeleted"
          @deleted="handleCreatedDeleted"
          @unselect="handleUnselect"
          class="sticky-card"
      /></v-col>
    </v-row>
  </template>
  <v-overlay
    :model-value="loading"
    class="align-center justify-center"
    persistent
  >
    <v-progress-circular
      color="primary"
      indeterminate
      size="64"
    ></v-progress-circular>
  </v-overlay>
</template>

<script lang="ts">
import { storeToRefs } from "pinia";
import { useAppStore } from "@/store/app";

import AppsList from "../components/AppsList.vue";
import AppCard from "../components/AppCard.vue";

import { API, App, GetApps } from "../../entities";

const defApps: App[] = [];

export default {
  name: "Apps",
  components: {
    AppsList,
    AppCard,
  },
  setup() {
    const app = useAppStore();
    const { getSearchText } = storeToRefs(app);
    return { getSearchText };
  },
  data() {
    return {
      app: {},
      apps: defApps,
      windowWidth: window.innerWidth,
      dialog: false,
      unselect: false,
      loading: false,
      firstLoad: true,
    };
  },
  created() {
    this.getApps();
  },
  mounted() {
    this.$nextTick(() => {
      window.addEventListener("resize", this.onResize);
    });
  },
  beforeDestroy() {
    window.removeEventListener("resize", this.onResize);
  },
  watch: {
    getSearchText(newValue, _oldValue) {
      console.log(newValue);
    },
  },
  computed: {
    compactMode() {
      return this.isCompactMode();
    },
  },
  methods: {
    openNew: function () {
      this.dialog = true;
    },
    getApps: function () {
      this.loading = true;

      this.axios
        .get<API<GetApps>>("/api/v1/apps")
        .then(({ data }) => {
          this.loading = false;

          if (
            typeof data.status !== "undefined" &&
            data.status === "done" &&
            data.data
          ) {
            this.apps = data.data;
          } else {
            const app = useAppStore();
            app.setErrorMessage("Error loading apps list!");
          }
        })
        .catch(({ response }) => {
          this.loading = false;

          const app = useAppStore();
          app.setErrorMessage("Error loading apps list!");
        });
    },
    isCompactMode: function () {
      return this.windowWidth < 960;
    },
    handleSelection: function (app: App) {
      this.app = app;

      if (this.isCompactMode() && !this.dialog && !this.firstLoad) {
        this.dialog = true;
      }

      this.firstLoad = false;
    },
    handleCreatedDeleted: function () {
      this.dialog = false;
      this.unselect = !this.unselect;
      this.getApps();
    },
    handleUpdated: function () {
      this.dialog = false;
      this.getApps();
    },
    handleUnselect: function () {
      this.unselect = !this.unselect;
    },
    handleClose: function () {
      this.dialog = false;
    },
    onResize: function () {
      this.windowWidth = window.innerWidth;
    },
  },
};
</script>

<style scoped>
.sticky-card {
  position: sticky;
  top: 68px;
}
</style>
