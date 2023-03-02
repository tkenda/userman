<template>
  <v-card class="mx-auto px-6 py-6" max-width="344" style="background: lightgray;">
    <v-form v-model="form" @submit.prevent="onSubmit">
      <div class="text-subtitle-1 text-medium-emphasis">Username</div>
      
      <v-text-field
        v-model="username"
        :readonly="loading"
        :rules="[required]"
        density="compact"
        variant="solo"
        autocomplete="username"
        prepend-inner-icon="mdi-account"
      ></v-text-field>

      <div class="text-subtitle-1 text-medium-emphasis">Password</div>

      <v-text-field
        v-model="password"
        :readonly="loading"
        :rules="[required]"
        :append-inner-icon="visible ? 'mdi-eye-off' : 'mdi-eye'"
        :type="visible ? 'text' : 'password'"
        density="compact"
        variant="solo"
        autocomplete="current-password"
        prepend-inner-icon="mdi-lock-outline"
        @click:append-inner="visible = !visible"
      ></v-text-field>

      <v-checkbox
        v-model="signedIn"
        color="secondary"
        label="Keep me signed in"
        density="compact"
      ></v-checkbox>

      <v-btn
        :disabled="!form"
        :loading="loading"
        color="primary"
        size="large"
        type="submit"
        variant="elevated"
        block
      >
        Sign In
      </v-btn>

      <br />

      <div class="d-flex justify-end">
        <a href="" class="text-decoration-none">Forgot password?</a>
      </div>

      <v-alert
        v-if="error"
        :text="error"
        class="mt-6"
        type="error"
        variant="tonal"
        @click="close"
      ></v-alert>
    </v-form>
  </v-card>
</template>

<script lang="ts">
import { API, PostLogin } from "../../entities";
import { useAuthStore } from "@/store/auth";

export default {
  name: "LoginCard",
  data: () => ({
    form: false,
    username: null,
    password: null,
    loading: false,
    signedIn: true,
    visible: false,
    error: "",
  }),
  methods: {
    onSubmit: function () {
      if (!this.form) return;
      this.loading = true;

      this.axios
        .post<API<PostLogin>>("/api/v1/login", {
          username: this.username,
          password: this.password,
        })
        .then(({ data }) => {
          this.loading = false;

          if (
            typeof data.status !== "undefined" &&
            data.status === "done" &&
            data.data
          ) {
            const auth = useAuthStore();
            const payload = data.data;

            auth.login(
              {
                username: this.username || "",
                ...payload,
              },
              !this.signedIn
            );
          } else {
            this.error = "Unknown error";
          }
        })
        .catch(({ response }) => {
          this.loading = false;

          if (
            typeof response.data.status !== "undefined" &&
            response.data.status === "error"
          ) {
            this.error = response.data.error;
          } else {
            this.error = "Unknown error";
          }
        });
    },
    required: function (v: any) {
      return !!v || "Field is required";
    },
    close: function () {
      this.error = "";
    },
  },
};
</script>

<style scoped>
</style>
