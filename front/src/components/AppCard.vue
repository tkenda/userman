<template>
  <v-card min-width="400px" :loading="loadingColor">
    <v-toolbar flat color="primary" density="compact">
      <v-toolbar-title class="font-weight-light">
        <strong v-html="name"></strong>
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
          v-model="appId"
          label="ID"
          variant="underlined"
          readonly
        ></v-text-field>
        <v-text-field
          v-model="localApp.name"
          label="Name"
          variant="underlined"
          :rules="[rules.required]"
        ></v-text-field>
        <v-text-field
          v-model.number="localApp.version"
          label="Version"
          variant="underlined"
          type="number"
        ></v-text-field>
        <v-row class="pb-2">
          <v-col class="text-right borderless-column">
            <v-btn variant="text" color="primary" @click="copyJson()">
              <span>Copy</span>
            </v-btn>
            <v-btn variant="text" color="primary" @click="editMode = !editMode">
              <span v-if="editMode">View</span>
              <span v-else>Edit</span>
            </v-btn>
          </v-col>
        </v-row>
      </v-form>
      <v-textarea
        v-if="editMode"
        variant="outlined"
        v-model="textJson"
        rows="10"
        :messages="errorMessages"
        :error="errorMessages !== undefined"
      ></v-textarea>
      <json-viewer
        v-else
        :value="localApp.defaultRole"
        :expand-depth="5"
        boxed
        sort
      ></json-viewer>
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
        @click="saveApp()"
        :disabled="!valid || errorMessages !== undefined"
        >Save</v-btn
      >
      <v-btn
        v-if="closeButton && permissions.delete"
        color="red"
        @click="deleteApp()"
        :disabled="!localApp.id"
        >Delete</v-btn
      >
      <v-spacer />
      <v-btn
        v-if="!closeButton && permissions.delete"
        color="red"
        @click="deleteApp()"
        :disabled="!localApp.id"
        >Delete</v-btn
      >
      <v-btn v-if="closeButton" color="gray" @click="close()">Close</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts">
import JsonViewer from "vue-json-viewer";

import { useAppStore } from "@/store/app";
import { App } from "../../entities";

const defApp: App = {
  name: undefined,
  defaultRole: [],
  version: 1,
  enabled: true,
};

const requiredRule = (value: string) => !!value || "This field is required.";

const clone = (obj: any) => {
  if (null == obj || "object" != typeof obj) return obj;
  var copy = obj.constructor();
  for (var attr in obj) {
    if (obj.hasOwnProperty(attr)) copy[attr] = obj[attr];
  }
  return copy;
};

type errorMessages = string | string[] | undefined;

export default {
  name: "AppCard",
  emit: ["updated", "created", "deleted", "close"],
  components: {
    JsonViewer,
  },
  props: {
    app: {
      default: {},
      type: Object,
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
      textJson: "",
      editMode: false,
      localApp: clone(defApp) as App,
      cardLoading: false,
      errorMessages: undefined as errorMessages,
      valid: false,
      appId: "",
      rules: {
        required: requiredRule,
      },
    };
  },
  watch: {
    app: {
      handler(newValue: App, _oldValue) {
        if (newValue.name !== undefined) {
          this.appId = newValue.id || "";
          this.localApp = clone(newValue) as App;
        } else {
          this.appId = "<NEW>";
          this.localApp = clone(defApp) as App;
        }

        this.textJson = JSON.stringify(this.localApp.defaultRole, null, 2);
      },
      deep: true,
      immediate: true,
    },
    textJson: {
      handler(newValue: string, _oldValue) {
        try {
          this.localApp.defaultRole = JSON.parse(newValue);
          this.errorMessages = undefined;
        } catch (err: unknown) {
          if (typeof err === "string") {
            this.errorMessages = err;
          } else if (err instanceof Error) {
            this.errorMessages = err.message;
          }
        }
      },
      deep: true,
      immediate: true,
    },
  },
  computed: {
    name() {
      return this.localApp.name === undefined ? "APP" : this.localApp.name;
    },
    loadingColor() {
      if (this.cardLoading) {
        return "grey";
      } else {
        return false;
      }
    },
  },
  methods: {
    clear: function () {
      this.$emit("unselect");
    },
    saveApp: function () {
      this.cardLoading = true;

      if (this.localApp.id) {
        this.axios
          .put("/api/v1/apps/" + this.localApp.id, this.localApp)
          .then(({ data }) => {
            this.cardLoading = false;

            if (typeof data.status === "undefined" || data.status !== "done") {
              const app = useAppStore();
              app.setErrorMessage("Error updating app!");
            }

            this.$emit("updated");
          })
          .catch(({ response }) => {
            this.cardLoading = false;
            const app = useAppStore();
            app.setErrorMessage("Error updating app!");
          });
      } else {
        this.axios
          .post("/api/v1/apps", this.localApp)
          .then(({ data }) => {
            this.cardLoading = false;

            if (typeof data.status === "undefined" || data.status !== "done") {
              const app = useAppStore();
              app.setErrorMessage("Error saving new app!");
            }

            this.$emit("created");
          })
          .catch(({ response }) => {
            this.cardLoading = false;
            const app = useAppStore();
            app.setErrorMessage("Error saving new app!");
          });
      }
    },
    deleteApp: function () {
      this.cardLoading = true;

      if (this.localApp.id) {
        this.axios
          .delete("/api/v1/apps/" + this.localApp.id)
          .then(({ data }) => {
            this.cardLoading = false;

            if (typeof data.status === "undefined" || data.status !== "done") {
              const app = useAppStore();
              app.setErrorMessage("Error deleting app!");
            }

            this.$emit("deleted");
          })
          .catch(({ response }) => {
            this.cardLoading = false;
            const app = useAppStore();
            app.setErrorMessage("Error deleting app!");
          });
      }
    },
    copyJson: function () {
      console.log("COPY");
    },
    close: function () {
      this.$emit("close");
    },
  },
};
</script>

<style scoped></style>
