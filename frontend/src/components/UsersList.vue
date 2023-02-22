<template>
  <v-card class="mx-auto" style="min-width: 400px">
    <v-list v-model:selected="selected" lines="two" :class="listStyle">
      <template v-for="department in localDepartments">
        <v-list-subheader class="title">{{ department.name }}</v-list-subheader>
        <template v-for="user in department.users" :key="user.id">
          <v-list-item
            :value="user"
            :title="title(user)"
            :subtitle="user.username"
            link
          >
            <template v-slot:prepend>
              <avatar :avatar="user.avatar" :username="user.username"></avatar>
            </template>
            <template v-slot:append>
              <v-btn
                color="primary"
                icon="mdi-content-copy"
                variant="text"
              ></v-btn>
              <v-btn
                color="primary"
                icon="mdi-lock-reset"
                variant="text"
              ></v-btn>
              <v-btn
                v-if="user.enabled"
                color="grey-lighten-1"
                icon="mdi-account-cancel"
                variant="text"
              ></v-btn>
              <v-btn
                v-else
                color="red-lighten-1"
                icon="mdi-account-cancel"
                variant="text"
              ></v-btn>
            </template>
          </v-list-item>
          <v-divider></v-divider>
        </template>
      </template>
    </v-list>
  </v-card>
</template>

<script lang="ts">
import Avatar from "./Avatar.vue";
import { Department, User } from "../../entities";

const defDepartments: Department[] = [];

export default {
  name: "UsersList",
  components: {
    Avatar,
  },
  props: {
    departments: {
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
    departments: {
      handler(newValue: Department[], _oldValue) {
        this.localDepartments = newValue;
      },
      deep: true,
      immediate: true,
    },
    selected: {
      handler(newValue: User[], _oldValue) {
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
      localDepartments: defDepartments,
    };
  },
  methods: {
    title: function (src: User) {
      return src.name + " " + src.surname;
    },
  },
};
</script>

<style scoped>
.title {
  background: #016060;
  color: #fff;
  height: 48px;
}
</style>
