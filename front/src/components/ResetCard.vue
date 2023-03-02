<template>
  <v-card
    class="mx-auto px-6 py-6"
    max-width="344"
    style="background: lightgray"
  >
    <v-form v-model="form" @submit.prevent="onSubmit">
      <div class="text-subtitle-1 text-medium-emphasis">New Password</div>

      <v-text-field
        v-model="password1"
        :readonly="loading"
        :rules="[required]"
        :append-inner-icon="visible1 ? 'mdi-eye-off' : 'mdi-eye'"
        :type="visible1 ? 'text' : 'password'"
        density="compact"
        variant="solo"
        autocomplete="current-password"
        prepend-inner-icon="mdi-lock-outline"
        @click:append-inner="visible1 = !visible1"
      ></v-text-field>

      <div class="text-subtitle-1 text-medium-emphasis">Repeat Password</div>

      <v-text-field
        v-model="password2"
        :readonly="loading"
        :rules="[required]"
        :append-inner-icon="visible2 ? 'mdi-eye-off' : 'mdi-eye'"
        :type="visible2 ? 'text' : 'password'"
        density="compact"
        variant="solo"
        autocomplete="current-password"
        prepend-inner-icon="mdi-lock-outline"
        @click:append-inner="visible2 = !visible2"
      ></v-text-field>

      <v-btn
        :disabled="!form"
        :loading="loading"
        color="primary"
        size="large"
        type="submit"
        variant="elevated"
        class="mt-2 mb-2"
        block
      >
        Save
      </v-btn>

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

export default {
  name: "ResetCard",
  props: {
    id: {
      required: true,
      type: String,
    },
  },
  data: () => ({
    form: false,
    password1: null,
    password2: null,
    visible1: false,
    visible2: false,
    loading: false,
    error: "",
  }),
  methods: {
    onSubmit: function () {
      if (!this.form) return;
      this.loading = true;

      if (this.password1 === this.password2) {
        this.axios
          .post<API<PostLogin>>("/api/v1/reset", {
            id: this.id,
            password: this.password1,
          })
          .then(({ data }) => {
            this.loading = false;

            if (typeof data.status !== "undefined" && data.status === "done") {
              this.$router.push("/login");
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
      } else {
        this.loading = false;
        this.error = "Password mismatch!";
      }
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

<style scoped></style>
