<template>
  <template v-if="compactMode">
    <v-row no-gutters>
      <v-col>
        <v-btn class="mt-2 mx-2" color="primary" @click="openNew">
          New
          <v-icon end icon="mdi-plus"></v-icon>
        </v-btn>
        <users-list
          :departments="departments"
          :unselect="unselect"
          @selected="handleSelection"
        />
      </v-col>
    </v-row>
    <v-dialog v-model="dialog">
      <user-card
        :user="user"
        :departments="departments"
        :roleNames="roleNames"
        @created="handleCreatedDeleted"
        @deleted="handleCreatedDeleted"
        @updated="handleUpdated"
        @unselect="handleUnselect"
        @close="handleClose"
        closeButton
      />
    </v-dialog>
  </template>
  <template v-else>
    <v-row no-gutters>
      <v-col style="padding: 20px 0px 20px 20px">
        <users-list
          :departments="departments"
          :unselect="unselect"
          @selected="handleSelection"
          removePadding
        />
      </v-col>
      <v-col style="padding: 20px">
        <user-card
          :user="user"
          :departments="departments"
          :roleNames="roleNames"
          @created="handleCreatedDeleted"
          @deleted="handleCreatedDeleted"
          @updated="handleUpdated"
          @unselect="handleUnselect"
          class="sticky-card"
          newButton
        />
      </v-col>
    </v-row>
  </template>
  <v-overlay
    :model-value="loadingUsers || loadingRoleNames"
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

import UsersList from "../components/UsersList.vue";
import UserCard from "../components/UserCard.vue";

import {
  API,
  GetUsers,
  Department,
  User,
  RoleName,
  GetRoleNames,
} from "../../entities";

const defDeps: Department[] = [];
const defRoleNames: RoleName[] = [];

export default {
  name: "Users",
  components: {
    UsersList,
    UserCard,
  },
  setup() {
    const app = useAppStore();
    const { getSearchText } = storeToRefs(app);
    return { getSearchText };
  },
  data() {
    return {
      user: {},
      roleNames: defRoleNames,
      departments: defDeps,
      windowWidth: window.innerWidth,
      dialog: false,
      unselect: false,
      loadingUsers: false,
      loadingRoleNames: false,
      firstLoad: true,
    };
  },
  created() {
    this.getUsers();
    this.getRoleNames();
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
    getUsers: function () {
      this.loadingUsers = true;

      this.axios
        .get<API<GetUsers>>("/api/v1/users")
        .then(({ data }) => {
          this.loadingUsers = false;

          if (
            typeof data.status !== "undefined" &&
            data.status === "done" &&
            data.data
          ) {
            const payload = data.data;

            let departments = [];

            for (let i = 0; i < payload.length; i++) {
              const item = departments.find(
                (el) => el.name == payload[i].department
              );

              if (item) {
                item.users.push(payload[i]);
              } else {
                departments.push({
                  name: payload[i].department || "<EMPTY>",
                  users: [payload[i]],
                });
              }
            }

            this.departments = departments;
          } else {
            const app = useAppStore();
            app.setErrorMessage("Error loading users list!");
          }
        })
        .catch(({ response }) => {
          this.loadingUsers = false;
          const app = useAppStore();
          app.setErrorMessage("Error loading users list!");
        });
    },
    getRoleNames: function () {
      this.loadingRoleNames = true;

      this.axios
        .get<API<GetRoleNames>>("/api/v1/rolenames")
        .then(({ data }) => {
          this.loadingRoleNames = false;

          if (
            typeof data.status !== "undefined" &&
            data.status === "done" &&
            data.data
          ) {
            this.roleNames = data.data;
          } else {
            const app = useAppStore();
            app.setErrorMessage("Error loading rolenames list!");
          }
        })
        .catch(({ response }) => {
          this.loadingRoleNames = false;
          const app = useAppStore();
          app.setErrorMessage("Error loading rolenames list!");
        });
    },
    isCompactMode: function () {
      return this.windowWidth < 960;
    },
    handleSelection: function (user: User) {
      this.user = user;

      if (
        this.isCompactMode() &&
        !this.dialog &&
        user.id !== undefined &&
        !this.firstLoad
      ) {
        this.dialog = true;
      }

      this.firstLoad = false;
    },
    handleCreatedDeleted: function () {
      this.dialog = false;
      this.unselect = !this.unselect;
      this.getUsers();
    },
    handleUpdated: function () {
      this.dialog = false;
      this.getUsers();
    },
    handleUnselect: function () {
      this.unselect = !this.unselect;
    },
    handleClose: function () {
      this.unselect = !this.unselect;
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
