<template>
  <v-card v-for="item in items" variant="tonal" density="compact" class="mb-2">
    <template v-slot:title>{{ item.name }}</template>
    <template v-slot:text>
      <role-values :values="item.values" />
      <role-root-a :items="item.items" />
    </template>
  </v-card>
</template>

<script lang="ts">
import RoleValues from "./RoleValues.vue";
import RoleRootA from "./RoleRootA.vue";

export default {
  name: "RoleItemsA",
  components: {
    RoleValues,
    RoleRootA,
  },
  props: {
    items: {
      default: [],
      type: Object,
    },
  },
};
</script>
