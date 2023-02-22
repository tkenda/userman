import { createRouter, createWebHistory } from "vue-router";

import Login from "@/views/Login.vue";

import Default from "@/views/Default.vue";
import Users from "@/views/Users.vue";
import Roles from "@/views/Roles.vue";
import Apps from "@/views/Apps.vue";

import NotFound from "@/views/NotFound.vue";

import { useAuthStore } from "@/store/auth";

const routes = [
  {
    name: "Login",
    path: "/login",
    component: Login,
  },
  {
    path: "/",
    component: Default,
    children: [
      {
        name: "Users",
        path: "/users",
        component: Users,
      },
      {
        name: "Roles",
        path: "/roles",
        component: Roles,
      },
      {
        name: "Apps",
        path: "/apps",
        component: Apps,
      },
      {
        path: "/",
        redirect: () => {
          return "/users";
        },
      },
    ],
  },
  {
    name: "NotFound",
    path: "/:pathMatch(.*)*",
    component: NotFound,
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

// Redirect to login page if not logged in and trying to access a restricted page.
router.beforeEach(async (to) => {
  const publicPages = ["/login"];
  const authRequired = !publicPages.includes(to.path);
  const auth = useAuthStore();

  if (authRequired && !auth.isAuth) return "/login";
});

export default router;
