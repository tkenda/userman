<template>
  <template v-if="compactMode">
    <v-row no-gutters>
      <v-col>
        <v-btn class="mt-2 mx-2" color="primary" @click="openNew">
          New
          <v-icon end icon="mdi-plus"></v-icon>
        </v-btn>
        <roles-list
          :apps="apps"
          :roles="roles"
          :unselect="unselect"
          @selected="handleSelection"
        />
      </v-col>
    </v-row>
    <v-dialog v-model="dialog">
      <role-card
        :apps="apps"
        :role="role"
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
        <roles-list
          :apps="apps"
          :roles="roles"
          :unselect="unselect"
          @selected="handleSelection"
          removePadding
        />
      </v-col>
      <v-col style="padding: 20px">
        <role-card
          :apps="apps"
          :role="role"
          class="sticky-card"
          @updated="handleUpdated"
          @created="handleCreatedDeleted"
          @deleted="handleCreatedDeleted"
          @unselect="handleUnselect"
          newButton
        />
      </v-col>
    </v-row>
  </template>
  <v-overlay
    :model-value="rolesLoading || appsLoading"
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

import RolesList from "../components/RolesList.vue";
import RoleCard from "../components/RoleCard.vue";

import { API, App, GetApps, Role, RoleApp, GetRoles } from "../../entities";

const defRoleApps: RoleApp[] = [];
const defApps: App[] = [];

export default {
  name: "Roles",
  components: {
    RolesList,
    RoleCard,
  },
  setup() {
    const app = useAppStore();
    const { getSearchText } = storeToRefs(app);
    return { getSearchText };
  },
  data() {
    return {
      role: {},
      roles: defRoleApps,
      apps: defApps,
      windowWidth: window.innerWidth,
      dialog: false,
      unselect: false,
      rolesLoading: false,
      appsLoading: false,
      firstLoad: true,
    };
  },
  created() {
    this.getApps();
    this.getRoles();
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
      this.appsLoading = true;

      this.axios
        .get<API<GetApps>>("/api/v1/apps")
        .then(({ data }) => {
          this.appsLoading = false;

          if (
            typeof data.status !== "undefined" &&
            data.status === "done" &&
            data.data
          ) {
            this.apps = data.data;
          } else {
            const app = useAppStore();
            app.setErrorMessage("Error loading roles list!");
          }
        })
        .catch(({ response }) => {
          this.appsLoading = false;
          const app = useAppStore();
          app.setErrorMessage("Error loading roles list!");
        });
    },
    getRoles: function () {
      this.rolesLoading = true;

      this.axios
        .get<API<GetRoles>>("/api/v1/roles")
        .then(({ data }) => {
          this.rolesLoading = false;

          if (
            typeof data.status !== "undefined" &&
            data.status === "done" &&
            data.data
          ) {
            const payload = data.data;

            let roles: RoleApp[] = [];

            for (let i = 0; i < payload.length; i++) {
              const item = roles.find((el) => el.id == payload[i].app);

              if (item) {
                item.roles.push(payload[i]);
              } else {
                roles.push({
                  id: payload[i].app || "",
                  roles: [payload[i]],
                });
              }
            }

            this.roles = roles;
          } else {
            const app = useAppStore();
            app.setErrorMessage("Error loading roles list!");
          }
        })
        .catch(({ response }) => {
          this.rolesLoading = false;
          const app = useAppStore();
          app.setErrorMessage("Error loading roles list!");
        });
    },
    isCompactMode: function () {
      return this.windowWidth < 960;
    },
    handleSelection: function (role: Role) {
      this.role = role;

      if (this.isCompactMode() && !this.dialog && !this.firstLoad) {
        this.dialog = true;
      }

      this.firstLoad = false;
    },
    handleCreatedDeleted: function () {
      this.dialog = false;
      this.unselect = !this.unselect;
      this.getApps();
      this.getRoles();
    },
    handleUpdated: function () {
      this.dialog = false;
      this.getApps();
      this.getRoles();
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
