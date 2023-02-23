import axios from "axios";

import { API, PostRefresh } from "../../entities";
import { useAuthStore } from "@/store/auth";

let isFetchingToken = false;
let tokenSubscribers: any[] = [];

function subscribeTokenRefresh(cb: any) {
  tokenSubscribers.push(cb);
}

function onTokenRefreshed(errRefreshing: any, token: string | null) {
  tokenSubscribers.map((cb) => cb(errRefreshing, token));
}

const instance = axios.create();

instance.interceptors.request.use((config) => {
  const auth = useAuthStore();

  if (auth.accessToken) {
    config.headers.Authorization = `Bearer ${auth.accessToken}`;
    config.headers["Content-Type"] = "application/json";
  } else {
    auth.logout();
  }

  return config;
});

instance.interceptors.response.use(
  (response) => {
    return response;
  },
  (error) => {
    const auth = useAuthStore();

    // if is not unauthorized reject.
    // !error.response => Connection error (net::ERR_CONNECTION_REFUSED)
    // error.response && error.response.status !== 401 => Error response
    if (!error.response || (error.response && error.response.status !== 401)) {
      return Promise.reject(error);
    }

    if (!isFetchingToken) {
      isFetchingToken = true;

      axios
        .post<API<PostRefresh>>("/api/v1/refresh", {
          username: auth.username,
          refreshToken: auth.refreshToken,
        })
        .then(({ data }) => {
          isFetchingToken = false;

          if (
            typeof data.status !== "undefined" &&
            data.status === "done" &&
            data.data
          ) {
            // Apply new token to the requests.
            auth.setAccessToken(data.data.accessToken);
            onTokenRefreshed(null, "Bearer " + data.data.accessToken);
          } else {
            // If response is tampered, logout with error message.
            onTokenRefreshed(new Error(data.error), null);
            auth.logout();
          }

          tokenSubscribers = [];
        })
        .catch((error) => {
          if (
            typeof error.response.data.status !== "undefined" &&
            error.response.data.status === "error"
          ) {
            onTokenRefreshed(new Error(error.response.data.error), null);
            tokenSubscribers = [];
            auth.logout();
          } else {
            onTokenRefreshed(new Error(error), null);
            tokenSubscribers = [];
          }
        });
    }

    const initTokenSubscriber = new Promise((resolve, reject) => {
      subscribeTokenRefresh((errRefreshing: any, newToken: string) => {
        if (errRefreshing) return reject(errRefreshing);

        // Update header in failed requests.
        error.config.headers.Authorization = newToken;

        // Only try 5 times to get an authorization.
        if ("Retried-Times" in error.config.headers) {
          if (error.config.headers["Retried-Times"] >= 5) {
            return reject("Max retries getting a new token!");
          }

          error.config.headers["Retried-Times"] =
            error.config.headers["Retried-Times"] + 1;
        } else {
          error.config.headers["Retried-Times"] = 1;
        }

        return resolve(axios(error.config));
      });
    });

    return initTokenSubscriber;
  }
);

export default instance;
