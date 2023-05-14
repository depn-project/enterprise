import { Theme } from "./model";

export const THEME_SWITCHER_CONFIG: Array<{ value: Theme; icon?: string; default?: boolean }> = [
  { value: "dark", icon: "Moon" },
  { value: "light", icon: "Sun" },
];
