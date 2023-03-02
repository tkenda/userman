import { defineStore } from "pinia";

export const useAppStore = defineStore("app", {
  state: () => ({
    searchText: "",
    errorMessage: "",
    infoMessage: "",
  }),
  getters: {
    getSearchText: (state) => {
      return state.searchText;
    },
    getErrorMessage: (state) => {
      return state.errorMessage;
    },
    getInfoMessage: (state) => {
      return state.infoMessage;
    },
  },
  actions: {
    setSearchText(src: string) {
      this.searchText = src;
    },
    setErrorMessage(src: string) {
      this.errorMessage = src;
      window.setTimeout(() => {
        this.errorMessage = "";
      }, 6000);
    },
    setInfoMessage(src: string) {
      this.infoMessage = src;
      window.setTimeout(() => {
        this.infoMessage = "";
      }, 2000);
    }
  },
});
