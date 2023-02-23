<template>
  <v-card class="mx-auto" style="min-width: 400px">
    <v-list v-model:selected="selected" lines="two" :class="listStyle">
      <template v-for="item in localRoles">
        <v-list-subheader class="title">{{ title(item.id) }}</v-list-subheader>
        <template v-for="(rol, i) in item.roles" :key="i">
          <v-list-item :value="rol" :title="rol.name" density="compact" link>
            <template v-slot:append>
              <v-btn
                v-if="!synced(rol)"
                color="warning"
                icon="mdi-sync-alert"
                variant="text"
              ></v-btn>
              <v-btn
                color="primary"
                icon="mdi-chevron-right"
                variant="text"
              ></v-btn>
            </template>
          </v-list-item>
          <v-divider></v-divider>
        </template>
      </template>
    </v-list>
  </v-card>
</template>

<script lang="ts">
import { App, Role, RoleApp } from "../../entities";

import { clearItems } from "../global";

const defRoleApps: RoleApp[] = [];
const defApps: App[] = [];

export default {
  name: "RolesList",
  props: {
    roles: {
      default: defRoleApps,
      type: Object,
    },
    apps: {
      default: defApps,
      type: Array,
    },
    removePadding: {
      default: false,
      type: Boolean,
    },
    unselect: {
      default: false,
      type: Boolean,
    },
  },
  computed: {
    listStyle() {
      if (this.removePadding) {
        return "py-0";
      } else {
        return "";
      }
    },
  },
  watch: {
    roles: {
      handler(newValue: RoleApp[], _oldValue) {
        this.localRoles = newValue;
      },
      deep: true,
      immediate: true,
    },
    selected: {
      handler(newValue: Role[], _oldValue) {
        if (newValue && newValue.length === 1) {
          this.$emit("selected", newValue[0]);
        } else {
          this.$emit("selected", {});
        }
      },
      deep: true,
      immediate: true,
    },
    unselect: {
      handler(_newValue: boolean, _oldValue) {
        this.selected = undefined;
      },
    },
  },
  data() {
    return {
      selected: undefined,
      localRoles: defRoleApps,
    };
  },
  methods: {
    title: function (src: string) {
      const app = this.apps.find((el: any) => el.id === src) as App;
      return app.name + " v." + app.version;
    },
    synced: function (role: Role) {
      const app = this.apps.find((el: any) => el.id === role.app) as App;

      const defaultRole = JSON.parse(JSON.stringify(app.defaultRole));
      const actualRole = JSON.parse(JSON.stringify(role.items));

      clearItems(defaultRole);
      clearItems(actualRole);

      return JSON.stringify(defaultRole) === JSON.stringify(actualRole);
    },
  },
};
</script>

<style scoped>
.title {
  background: #016060;
  color: #fff;
  height: 48px;
}
</style>
