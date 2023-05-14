<script setup lang="ts">
import Select from "@/shared/ui/Select";
import { LANGUAGE_SWITCHER_CONFIG, DEFAULT_LOCALE } from "./config";
import { LanguageLocale } from "./model";
import { changeLocale } from "./lib";
import { ref, watch } from "vue";

const selectedLocale = ref<LanguageLocale>(DEFAULT_LOCALE);

watch(selectedLocale, (locale) => {
  changeLocale(locale);
});
</script>

<template>
  <Select v-model="selectedLocale" :options="LANGUAGE_SWITCHER_CONFIG" />
</template>
