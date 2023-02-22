<template>
  <v-avatar v-if="avatar" :size="size">
    <v-img :src="avatar"></v-img>
  </v-avatar>
  <v-avatar :color="color" :size="size" v-else-if="username">
    <span class="text-h5" :size="size">
      {{ username.charAt(0).toUpperCase() }}
    </span>
  </v-avatar>
  <v-avatar :color="color" :size="size" v-else>
    <v-icon :size="size" dark>mdi-account-circle</v-icon>
  </v-avatar>
</template>

<script lang="ts">
export default {
  name: "Avatar",
  props: {
    avatar: {
      default: undefined,
      type: String,
    },
    username: {
      default: "",
      type: String,
    },
    size: {
      default: "default",
      type: String,
    },
    color: {
      default: "primary",
      type: String,
    }
  },
};
</script>

<style scoped></style>
