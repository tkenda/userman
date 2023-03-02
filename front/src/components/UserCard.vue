<template>
  <v-card min-width="400px" :loading="loadingColor">
    <v-toolbar flat color="primary" density="compact">
      <v-btn icon>
        <avatar
          :avatar="localUser.avatar"
          :username="localUser.username"
          color="grey"
          size="small"
        ></avatar>
      </v-btn>

      <v-toolbar-title class="font-weight-light">
        <strong v-html="fullname"></strong>
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

    <v-card-text class="pt-4 pb-2">
      <v-form v-model="valid" ref="form">
        <v-text-field
          v-model="userId"
          label="ID"
          variant="underlined"
          readonly
        ></v-text-field>
        <v-text-field
          v-model="localUser.username"
          label="Username"
          variant="underlined"
          :rules="[rules.required]"
          :loading="usernameLoading"
          :messages="uniqueMessage"
          :error="!uniqueUsername"
          @change="checkUsername"
        ></v-text-field>
        <v-text-field
          v-model="localUser.name"
          label="Name"
          variant="underlined"
          :rules="[rules.required]"
        ></v-text-field>
        <v-text-field
          v-model="localUser.surname"
          label="Surname"
          variant="underlined"
          :rules="[rules.required]"
        ></v-text-field>
        <v-text-field
          v-model="localUser.email"
          label="Mail"
          variant="underlined"
          type="email"
          :rules="[rules.required, rules.email]"
        ></v-text-field>
        <v-textarea
          v-model="localUser.description"
          label="Description"
          variant="underlined"
          :rules="[rules.required]"
          auto-grow
          rows="1"
        ></v-textarea>
        <v-combobox
          v-model="localUser.department"
          label="Department"
          variant="underlined"
          :items="departments.map((item: any) => { return item.name; })"
          :rules="[rules.required]"
        ></v-combobox>
        <v-select
          v-model="localUser.roles"
          :items="roleNames.map((item: any) => { 
            return {
              title: item.name,
              value: item.id,
            }
          })"
          label="Roles"
          variant="underlined"
          multiple
          chips
        ></v-select>
        <v-switch
          v-model="localUser.enabled"
          label="Enabled"
          color="primary"
          inset
        ></v-switch>
      </v-form>
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
        @click="saveUser()"
        :disabled="!valid"
        >Save</v-btn
      >
      <v-btn
        v-if="closeButton && permissions.update"
        :disabled="!localUser.id"
        color="primary"
        @click="resetPassword()"
        >Reset Password</v-btn
      >
      <v-btn
        v-if="closeButton && permissions.delete"
        :disabled="!localUser.id"
        color="red"
        @click="deleteUser()"
        >Delete</v-btn
      >
      <v-spacer />
      <v-btn
        v-if="!closeButton && permissions.update"
        :disabled="!localUser.id"
        color="primary"
        @click="resetPassword()"
        >Reset Password</v-btn
      >
      <v-btn
        v-if="!closeButton && permissions.delete"
        :disabled="!localUser.id"
        color="red"
        @click="deleteUser()"
        >Delete</v-btn
      >
      <v-btn v-if="closeButton" color="gray" @click="close()">Close</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts">
import Avatar from "./Avatar.vue";

import { useAppStore } from "@/store/app";
import { API, GetUsername, RoleName, User } from "../../entities";

const defUser: User = {
  username: undefined,
  name: undefined,
  surname: undefined,
  email: undefined,
  description: undefined,
  department: undefined,
  roles: [],
  enabled: true,
};

const defRoleNames: RoleName[] = [];

const requiredRule = (value: string) => !!value || "This field is required.";

const emailRule = (value: string) => {
  const pattern =
    /^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/;
  return pattern.test(value) || "Invalid e-mail.";
};

const clone = (obj: any) => {
  if (null == obj || "object" != typeof obj) return obj;
  var copy = obj.constructor();
  for (var attr in obj) {
    if (obj.hasOwnProperty(attr)) copy[attr] = obj[attr];
  }
  return copy;
};

export default {
  name: "UserCard",
  emit: ["updated", "created", "deleted", "close"],
  components: {
    Avatar,
  },
  props: {
    user: {
      default: {},
      type: Object,
    },
    departments: {
      default: [],
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
    roleNames: {
      default: defRoleNames,
      type: Array,
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
      localUser: clone(defUser) as User,
      cardLoading: false,
      usernameLoading: false,
      uniqueUsername: true,
      valid: false,
      userId: "",
      rules: {
        required: requiredRule,
        email: emailRule,
      },
      roles: [],
    };
  },
  watch: {
    user: {
      handler(newValue: User, _oldValue) {
        if (newValue.name !== undefined) {
          this.userId = newValue.id || "";
          this.localUser = clone(newValue);
        } else {
          this.userId = "<NEW>";
          this.localUser = clone(defUser);
        }
      },
      deep: true,
      immediate: true,
    },
  },
  mounted() {
    if (this.localUser.id) {
      const form = this.$refs.form as any;
      form.validate();
    }
  },
  computed: {
    fullname() {
      if (
        this.localUser.name === undefined ||
        this.localUser.surname === undefined
      ) {
        return "USER";
      } else {
        return this.localUser.name + " " + this.localUser.surname;
      }
    },
    loadingColor() {
      if (this.cardLoading) {
        return "grey";
      } else {
        return false;
      }
    },
    uniqueMessage: function () {
      if (this.uniqueUsername) {
        return undefined;
      } else {
        return "Username already exists!";
      }
    },
  },
  methods: {
    clear: function () {
      this.$emit("unselect");
    },
    clipboardCopy: function (text: string) {
      const app = useAppStore();

      if (typeof navigator.clipboard !== "undefined") {
        navigator.clipboard.writeText(text);
        app.setInfoMessage("Copied to clipboard!");
      } else {
        app.setInfoMessage("Your browser is blocking clipboard!");
      }
    },
    checkUsername: function () {
      if (this.localUser.username) {
        this.usernameLoading = true;

        this.axios
          .get<API<GetUsername>>("/api/v1/usernames/" + this.localUser.username)
          .then(({ data }) => {
            this.usernameLoading = false;

            if (typeof data.status !== "undefined" && data.status === "done") {
              this.uniqueUsername = typeof data.data === "undefined";
            } else {
              const app = useAppStore();
              app.setErrorMessage("Error checking username!");
            }
          })
          .catch(({ response }) => {
            this.usernameLoading = false;
            const app = useAppStore();
            app.setErrorMessage("Error checking username!");
          });
      }
    },
    saveUser: function () {
      this.cardLoading = true;

      if (this.localUser.id) {
        this.axios
          .put("/api/v1/users/" + this.localUser.id, this.localUser)
          .then(({ data }) => {
            this.cardLoading = false;

            if (typeof data.status === "undefined" || data.status !== "done") {
              const app = useAppStore();
              app.setErrorMessage("Error updating user!");
            }

            this.$emit("updated");
          })
          .catch(({ response }) => {
            this.cardLoading = false;
            const app = useAppStore();
            app.setErrorMessage("Error updating user!");
          });
      } else {
        this.axios
          .post("/api/v1/users", this.localUser)
          .then(({ data }) => {
            this.cardLoading = false;

            if (typeof data.status === "undefined" || data.status !== "done") {
              const app = useAppStore();
              app.setErrorMessage("Error saving new user!");
            }

            this.$emit("created");
          })
          .catch(({ response }) => {
            this.cardLoading = false;
            const app = useAppStore();
            app.setErrorMessage("Error saving new user!");
          });
      }
    },
    deleteUser: function () {
      this.cardLoading = true;

      if (this.localUser.id) {
        this.axios
          .delete("/api/v1/users/" + this.localUser.id)
          .then(({ data }) => {
            this.cardLoading = false;

            if (typeof data.status === "undefined" || data.status !== "done") {
              const app = useAppStore();
              app.setErrorMessage("Error deleting user!");
            }

            this.$emit("deleted");
          })
          .catch(({ response }) => {
            this.cardLoading = false;
            const app = useAppStore();
            app.setErrorMessage("Error deleting user!");
          });
      }
    },
    resetPassword: function () {
      this.cardLoading = true;

      if (this.localUser.id) {
        this.axios
          .get("/api/v1/users/" + this.localUser.id + "/reset")
          .then(({ data }) => {
            this.cardLoading = false;

            if (typeof data.status === "undefined" || data.status !== "done") {
              const app = useAppStore();
              app.setErrorMessage("Error resetting user password!");
            } else {
              let url = new URL("/reset/" + this.localUser.id, window.location.href);
              this.clipboardCopy(url.toString());

              this.$emit("updated");
            }
          })
          .catch(({ response }) => {
            this.cardLoading = false;
            const app = useAppStore();
            app.setErrorMessage("Error resetting user password!");
          });
      }
    },
    close: function () {
      this.$emit("close");
    },
  },
};
</script>

<style scoped></style>
