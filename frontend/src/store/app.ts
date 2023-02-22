import { defineStore } from "pinia";

export const useAppStore = defineStore("app", {
  state: () => ({
    searchText: "",
    errorMessage: "",
  }),
  getters: {
    getSearchText: (state) => {
      return state.searchText;
    },
    getErrorMessage: (state) => {
      return state.errorMessage;
    },
  },
  actions: {
    setSearchText(src: string) {
      this.searchText = src;
    },
    setErrorMessage(src: string) {
      this.errorMessage = src;
    },
  },
});
