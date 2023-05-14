import { createApp } from "vue";
import "@/app/styles.scss";
import App from "@/app";
import { i18n } from "@/shared/lib/i18n";

createApp(App).use(i18n).mount("#app");
