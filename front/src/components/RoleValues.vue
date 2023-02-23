<template>
  <v-container>
    <v-row>
      <template v-for="value in values">
        <template v-if="value.boolean !== undefined">
          <v-col cols="12" class="py-1">
            <v-switch
              inset
              hide-details
              v-model="value.boolean"
              :label="value.name"
              color="primary"
              density="compact"
            ></v-switch>
          </v-col>
        </template>
        <template v-else-if="value.float !== undefined">
          <v-col cols="12">
            <v-text-field
              hide-details
              variant="outlined"
              v-model.number="value.float"
              :label="value.name"
              type="number"
              density="compact"
            ></v-text-field>
          </v-col>
        </template>
        <template v-else-if="value.number !== undefined">
          <v-col cols="12">
            <v-text-field
              hide-details
              variant="outlined"
              v-model.number="value.number"
              :label="value.name"
              type="number"
              density="compact"
            ></v-text-field>
          </v-col>
        </template>
        <template v-else-if="value.string !== undefined">
          <v-col cols="12">
            <v-text-field
              hide-details
              variant="outlined"
              v-model="value.string"
              :label="value.name"
              density="compact"
            ></v-text-field>
          </v-col>
        </template>
      </template>
    </v-row>
  </v-container>
</template>

<script lang="ts">
export default {
  name: "RoleValues",
  props: {
    values: {
      default: [],
      type: Object,
    },
  },
};
</script>
