<template>
  <v-card min-width="400px" :loading="loadingColor">
    <v-toolbar flat color="primary" density="compact">
      <v-toolbar-title class="font-weight-light">
        <strong v-html="localRole.name"></strong>
      </v-toolbar-title>

      <v-toolbar-title
        class="d-flex justify-end px-2"
        v-if="newButton && permissions.create"
      >
        <v-btn variant="outlined" @click="clear">
          New
          <v-icon end icon="mdi-plus"></v-icon>
        </v-btn>
      </v-toolbar-title>
    </v-toolbar>

    <v-card-text class="overflow-auto">
      <v-form v-model="valid" ref="form">
        <v-text-field
          v-model="roleId"
          label="ID"
          variant="underlined"
          readonly
        ></v-text-field>
        <v-select
          v-model="localRole.app"
          label="App"
          variant="underlined"
          :rules="[rules.required]"
          :items="appsItem"
          @update:model-value="updateApp"
        ></v-select>
        <v-text-field
          v-model="localRole.name"
          label="Name"
          variant="underlined"
          :rules="[rules.required]"
          :loading="nameLoading"
          :messages="uniqueMessage"
          :error="!uniqueName"
          @change="checkName"
        ></v-text-field>
      </v-form>

      <v-label class="text-caption pb-2">Permissions</v-label>

      <v-expansion-panels>
        <v-expansion-panel v-for="item in localRole.items">
          <v-expansion-panel-title>{{ item.name }}</v-expansion-panel-title>
          <v-expansion-panel-text>
            <role-values :values="item.values" />
            <role-items-a :items="item.items" />
          </v-expansion-panel-text>
        </v-expansion-panel>
      </v-expansion-panels>
    </v-card-text>

    <v-card-actions
      v-if="permissions.create || permissions.update || permissions.delete"
    >
      <v-btn
        v-if="permissions.create || permissions.update"
        color="primary"
        @click="clear()"
        >Clear</v-btn
      >
      <v-btn
        v-if="permissions.create || permissions.update"
        color="primary"
        @click="saveRole()"
        :disabled="!valid"
        >Save</v-btn
      >
      <v-btn
        v-if="permissions.update"
        color="warning"
        @click="syncRole()"
        :disabled="!needSync"
        >Sync</v-btn
      >
      <v-btn
        v-if="closeButton && permissions.delete"
        :disabled="!localRole.id"
        color="red"
        @click="deleteRole()"
        >Delete</v-btn
      >
      <v-spacer />
      <v-btn
        v-if="!closeButton && permissions.delete"
        :disabled="!localRole.id"
        color="red"
        @click="deleteRole()"
        >Delete</v-btn
      >
      <v-btn v-if="closeButton" color="gray" @click="close()">Close</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts">
import RoleValues from "./RoleValues.vue";
import RoleItemsA from "./RoleItemsA.vue";

import { useAppStore } from "@/store/app";
import { Role, App, API, GetRoleName } from "../../entities";

import { clone, clearItems } from "../global";

const defRole: Role = {
  app: undefined,
  name: undefined,
  items: [],
  enabled: true,
};

const defApps: App[] = [];

const requiredRule = (value: string) => !!value || "This field is required.";

export default {
  name: "RoleCard",
  emit: ["updated", "created", "deleted", "close"],
  components: {
    RoleValues,
    RoleItemsA,
  },
  props: {
    role: {
      default: defRole,
      type: Object,
    },
    apps: {
      default: defApps,
      type: Array,
    },
    permissions: {
      default: {
        create: false,
        read: false,
        update: false,
        delete: false,
      },
      type: Object,
    },
    closeButton: {
      default: false,
      type: Boolean,
    },
    newButton: {
      default: false,
      type: Boolean,
    },
  },
  data() {
    return {
      localRole: clone(defRole) as Role,
      cardLoading: false,
      nameLoading: false,
      syncLoading: false,
      uniqueName: true,
      valid: false,
      roleId: "",
      firstLoad: true,
      rules: {
        required: requiredRule,
      },
      appsVersions: [],
      needSync: false,
    };
  },
  watch: {
    role: {
      handler(newValue: Role, _oldValue) {
        if (newValue.name !== undefined) {
          this.roleId = newValue.id || "";
          this.localRole = clone(newValue);

          const app = this.apps.find(
            (el: any) => el.id === this.localRole.app
          ) as App;

          const defaultRole = JSON.parse(JSON.stringify(app.defaultRole));
          const actualRole = JSON.parse(JSON.stringify(this.localRole.items));

          clearItems(defaultRole);
          clearItems(actualRole);

          this.needSync =
            JSON.stringify(defaultRole) !== JSON.stringify(actualRole);
        } else {
          this.roleId = "<NEW>";
          this.localRole = clone(defRole);
        }
      },
      deep: true,
      immediate: true,
    },
  },
  computed: {
    loadingColor() {
      if (this.cardLoading || this.syncLoading) {
        return "grey";
      } else {
        return false;
      }
    },
    uniqueMessage: function () {
      if (this.uniqueName) {
        return undefined;
      } else {
        return "Name already exists!";
      }
    },
    appsItem: function () {
      return this.apps.map((el: any): any => {
        return {
          title: el.name + " v." + el.version,
          value: el.id,
        };
      });
    },
  },
  methods: {
    clear: function () {
      this.$emit("unselect");
    },
    checkName: function () {
      if (this.localRole.name) {
        this.nameLoading = true;

        this.axios
          .get<API<GetRoleName>>("/api/v1/rolenames/" + this.localRole.name)
          .then(({ data }) => {
            this.nameLoading = false;

            if (typeof data.status !== "undefined" && data.status === "done") {
              this.uniqueName = typeof data.data === "undefined";
            } else {
              const app = useAppStore();
              app.setErrorMessage("Error checking name!");
            }
          })
          .catch(({ response }) => {
            this.nameLoading = false;
            const app = useAppStore();
            app.setErrorMessage("Error checking name!");
          });
      }
    },
    syncRole: function () {
      if (this.localRole.id) {
        this.syncLoading = true;

        this.axios
          .get("/api/v1/roles/" + this.localRole.id + "/sync")
          .then(({ data }) => {
            this.syncLoading = false;

            if (typeof data.status !== "undefined" && data.status === "done") {
              this.uniqueName = typeof data.data === "undefined";
            } else {
              const app = useAppStore();
              app.setErrorMessage("Error syncing role!");
            }

            this.$emit("updated");
          })
          .catch(({ response }) => {
            this.syncLoading = false;
            const app = useAppStore();
            app.setErrorMessage("Error syncing role!");
          });
      }
    },
    updateApp: function () {
      const selected = this.apps.find(
        (el: any) => el.id === this.localRole.app
      ) as App;

      if (selected) {
        this.localRole.items = selected.defaultRole;
      } else {
        this.localRole.items = [];
      }
    },
    saveRole: function () {
      this.cardLoading = true;

      if (this.localRole.id) {
        this.axios
          .put("/api/v1/roles/" + this.localRole.id, this.localRole)
          .then(({ data }) => {
            this.cardLoading = false;

            if (typeof data.status === "undefined" || data.status !== "done") {
              const app = useAppStore();
              app.setErrorMessage("Error updating role!");
            }

            this.$emit("updated");
          })
          .catch(({ response }) => {
            this.cardLoading = false;
            const app = useAppStore();
            app.setErrorMessage("Error updating role!");
          });
      } else {
        this.axios
          .post("/api/v1/roles", this.localRole)
          .then(({ data }) => {
            this.cardLoading = false;

            if (typeof data.status === "undefined" || data.status !== "done") {
              const app = useAppStore();
              app.setErrorMessage("Error saving new role!");
            }

            this.$emit("created");
          })
          .catch(({ response }) => {
            this.cardLoading = false;
            const app = useAppStore();
            app.setErrorMessage("Error saving new role!");
          });
      }
    },
    deleteRole: function () {
      this.cardLoading = true;

      if (this.localRole.id) {
        this.axios
          .delete("/api/v1/roles/" + this.localRole.id)
          .then(({ data }) => {
            this.cardLoading = false;

            if (typeof data.status === "undefined" || data.status !== "done") {
              const app = useAppStore();
              app.setErrorMessage("Error deleting role!");
            }

            this.$emit("deleted");
          })
          .catch(({ response }) => {
            this.cardLoading = false;
            const app = useAppStore();
            app.setErrorMessage("Error deleting role!");
          });
      }
    },
    close: function () {
      this.$emit("close");
    },
  },
};
</script>
