<template>
  <v-card class="mx-auto" style="min-width: 400px">
    <v-list v-model:selected="selected" lines="two" :class="listStyle">
      <template v-for="(app, i) in localApps" :key="i">
        <v-list-item :value="app" :title="title(app)" link>
          <template v-slot:append>
            <v-btn
              color="primary"
              icon="mdi-chevron-right"
              variant="text"
            ></v-btn>
          </template>
        </v-list-item>
        <v-divider></v-divider>
      </template>
    </v-list>
  </v-card>
</template>

<script lang="ts">
import { App } from "../../entities";

const defApps: App[] = [];

export default {
  name: "AppsList",
  props: {
    apps: {
      default: [],
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
    apps: {
      handler(newValue: App[], _oldValue) {
        this.localApps = newValue;
      },
      deep: true,
      immediate: true,
    },
    selected: {
      handler(newValue: App[], _oldValue) {
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
      localApps: defApps,
    };
  },
  methods: {
    title: (app: App) => {
      return app.name + " v." + app.version + "";
    }
  },
};
</script>

<style scoped>
.title {
  background: #016060;
  color: #fff;
}
</style>
