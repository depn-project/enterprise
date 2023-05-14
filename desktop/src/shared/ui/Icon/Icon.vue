<script setup lang="ts">
import * as icons from "@/shared/assets/icons";
import { computed } from "vue";

interface Props {
  name: string;
  size?: number;
}

interface IconMap {
  [key: string]: string;
}

const props = defineProps<Props>();

const iconComponent = computed(() => (icons as unknown as IconMap)[props.name]);
</script>

<template>
  <component
    class="icon"
    :style="`width: ${props.size}; height: ${props.size};`"
    :is="iconComponent"
    role="img"
  />
</template>
