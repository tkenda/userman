import { defineStore } from "pinia";

export interface Login {
  username: string;
  accessToken: string;
  refreshToken: string;
}

const getLocal = (name: string) => {
  return localStorage.getItem(name);
}

const getSession = (name: string) => {
  return sessionStorage.getItem(name);
}

const getStorage = (name: string) => {
  return getSession(name) || getLocal(name);
};

export const useAuthStore = defineStore("auth", {
  state: () => ({
    username: getStorage("username"),
    accessToken: getStorage("accessToken"),
    refreshToken: getStorage("refreshToken"),
  }),
  getters: {
    isAuth: (state) => {
      return (
        state.username !== null &&
        state.accessToken !== null &&
        state.refreshToken !== null
      );
    },
    getUser: (state) => {
      return ({
        username: state.username, 
      })
    }
  },
  actions: {
    login(login: Login, session: boolean) {
      let storage;

      if (session) {
        storage = sessionStorage;
      } else {
        storage = localStorage;
      }

      storage.setItem("username", login.username);
      storage.setItem("accessToken", login.accessToken);
      storage.setItem("refreshToken", login.refreshToken);

      this.username = login.username;
      this.accessToken = login.accessToken;
      this.refreshToken = login.refreshToken;

      this.router.push("/");
    },
    logout() {
      this.username = null;
      this.accessToken = null;
      this.refreshToken = null;

      sessionStorage.clear();
      localStorage.clear();

      this.router.push("/login");
    },
    setAccessToken(accessToken: string) {
      if (
        getSession("username") !== null || 
        getSession("accessToken") !== null ||
        getSession("refreshToken") !== null
      ) {
        sessionStorage.setItem("accessToken", accessToken);
      } else {
        localStorage.setItem("accessToken", accessToken);
      }
      
      this.accessToken = accessToken;
    }
  },
});
