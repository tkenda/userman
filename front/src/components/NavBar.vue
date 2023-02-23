<template>
  <v-navigation-drawer theme="dark" width="180" v-model="drawer">
    <v-list>
      <v-list-item class="mx-auto text-center">
        <avatar :avatar="avatar" :username="username" size="x-large"></avatar>
        <v-list-item-title class="text-white pt-2">
          {{ username }}
        </v-list-item-title>
      </v-list-item>
    </v-list>

    <v-divider></v-divider>

    <v-list>
      <v-list-item
        v-for="(item, i) in items"
        :key="i"
        :value="item"
        :to="item.link"
        active-color="primary"
        link
      >
        <template v-slot:prepend>
          <v-icon :icon="item.icon"></v-icon>
        </template>

        <v-list-item-title v-text="item.text"></v-list-item-title>
      </v-list-item>
    </v-list>

    <template v-slot:append>
      <div class="pa-2">
        <v-btn block @click="logout">Logout</v-btn>
      </div>
    </template>
  </v-navigation-drawer>
</template>

<script lang="ts">
import Avatar from "./Avatar.vue";
import { storeToRefs } from "pinia";
import { useAuthStore } from "@/store/auth";

export default {
  name: "NavBar",
  components: {
    Avatar,
  },
  props: {
    toggleNavbar: {
      type: Boolean,
      default: true,
    },
  },
  setup() {
    const auth = useAuthStore();
    const { getUser } = storeToRefs(auth);
    return { getUser };
  },
  data: () => ({
    drawer: true,
    avatar: undefined,
    username: "",
    items: [
      { text: "Users", icon: "mdi-account-multiple", link: "/users" },
      { text: "Roles", icon: "mdi-shield-account", link: "/roles" },
      { text: "Apps", icon: "mdi-cog", link: "/apps" },
    ],
  }),
  watch: {
    getUser: {
      handler(newValue, _oldValue) {
        this.username = newValue.username;
      },
      deep: true,
      immediate: true,
    },
    toggleNavbar: {
      handler(newValue, oldValue) {
        if (newValue !== oldValue) {
          this.drawer = !this.drawer;
        }
      },
    },
  },
  methods: {
    logout: function () {
      const auth = useAuthStore();
      auth.logout();
    },
  },
};
</script>
