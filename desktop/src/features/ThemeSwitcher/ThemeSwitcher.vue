<script setup lang="ts">
import Switch from "@/shared/ui/Switch";
import { THEME_SWITCHER_CONFIG } from "./config";
import { useI18n } from "vue-i18n";
import { useDark } from "@vueuse/core";
import { watch, ref } from "vue";
import { Theme } from "./model";

const { t } = useI18n();
const isDark = useDark();
const value = ref<Theme>(isDark.value ? "dark" : "light");

watch(value, (value) => {
  isDark.value = value === "dark";
});
</script>

<template>
  <Switch
    v-model="value"
    :options="
      THEME_SWITCHER_CONFIG.map((theme) => ({
        ...theme,
        name: t(`feature.themeSwitcher.${theme.value}`),
      }))
    "
  />
</template>
