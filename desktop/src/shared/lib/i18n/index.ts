import { init, addMessages } from "svelte-i18n";
import be from "./be.json";
import en from "./en.json";

addMessages("be", be);
addMessages("en", en);

init({
  fallbackLocale: "be",
  initialLocale: "be",
});
