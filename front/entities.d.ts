/* Users */

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
  avatar?: string;
}

export interface Department {
  name: string;
  users: User[];
}

/* Roles */

export interface DataOptions {
  minValue: any;
  maxValue: any;
}

export interface Value {
  name: string;
  string?: string;
  float?: number;
  number?: number;
  boolean?: boolean;
  options?: DataOptions;
}

export interface Item {
  name: string;
  values?: Value[];
  items?: Item[];
}

export interface Role {
  id?: string;
  app: string | undefined;
  name: string | undefined;
  items: Item[];
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
  defaultRole: Item[];
  version: number;
  enabled: boolean;
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
