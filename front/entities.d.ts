/* Users */

import * as Userman from "userman-auth";

export interface User {
  id?: string;
  username: string | undefined;
  name: string | undefined;
  surname: string | undefined;
  email: string | undefined;
  description: string | undefined;
  department: string | undefined;
  enabled: boolean;
  roles: string[];
  password?: string;
  avatar?: string;
}

export interface Department {
  name: string;
  users: User[];
}

/* Roles */

export interface Role {
  id?: string;
  app: string | undefined;
  name: string | undefined;
  items: Userman.Item[];
  enabled: boolean;
}

export interface RoleApp {
  id: string;
  roles: Role[];
}

export interface RoleName {
  id: string;
  name: string | undefined;
}

/* Apps */

export interface App {
  id?: string;
  name: string | undefined;
  defaultRole: Userman.Item[];
  version: number;
  enabled: boolean;
}

/* Permissions */

export interface CRUD {
  create: boolean;
  read: boolean;
  update: boolean;
  delete: boolean;
}

/* API */

export interface API<T> {
  status: string;
  data?: T;
  error?: string;
}

export interface PostLogin {
  accessToken: string;
  refreshToken: string;
  permissions: Userman.Item[];
}

export interface PostRefresh {
  accessToken: string;
}

export type GetUsers = User[];

export type GetUsername = User;

export type GetApps = App[];

export type GetRoles = Role[];

export type GetRoleName = Role;

export type GetRoleNames = RoleName[];