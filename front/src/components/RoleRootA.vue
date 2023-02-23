<template>
  <v-expansion-panels>
    <v-expansion-panel v-for="item in items">
      <v-expansion-panel-title>{{ item.name }}</v-expansion-panel-title>
      <v-expansion-panel-text>
        <role-values :values="item.values" />
        <role-items-b :items="item.items" />
      </v-expansion-panel-text>
    </v-expansion-panel>
  </v-expansion-panels>
</template>

<script lang="ts">
import RoleValues from "./RoleValues.vue";
import RoleItemsB from "./RoleItemsB.vue";

export default {
  name: "RoleRootA",
  components: {
    RoleValues,
    RoleItemsB,
  },
  props: {
    items: {
      default: [],
      type: Object,
    },
  }
};
</script>
