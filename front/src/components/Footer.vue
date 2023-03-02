<template>
  <v-footer color="#061D53">
    <v-row justify="center" no-gutters>
      <v-col class="text-center" cols="12">
        <strong style="color: #ededed">{{ text }}</strong>
      </v-col>
    </v-row>
  </v-footer>
</template>

<script lang="ts">
export default {
  name: "Footer",
  data: () => ({
    text: "User Managment Tool " + new Date().getFullYear(),
  }),
};
</script>
