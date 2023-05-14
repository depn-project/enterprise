<script setup lang="ts">
import { computed, onBeforeMount, ref, watch } from "vue";
import { onClickOutside } from "@vueuse/core";
import { ChevronDown } from "@/shared/assets/icons";
import Icon from "@/shared/ui/Icon";

interface Option {
  value: string;
  label?: string;
  icon?: string;
  default?: boolean;
}

interface Props {
  options: Array<Option>;
  modelValue?: string;
  placeholder?: string;
}

interface Emits {
  (e: "update:modelValue", payload: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const select = ref();
const value = ref<string>(props.modelValue || "");

const collapsed = ref<boolean>(true);
const toggle = () => (collapsed.value = !collapsed.value);
const iconRotate = computed(() => `${collapsed.value ? 0 : 180}deg`);

onClickOutside(select, () => (collapsed.value = true));

watch(value, (value) => emit("update:modelValue", value));

const getOptionIcon = (value: string) =>
  props.options.find((option) => option.value === value)?.icon;

const getOptionLabel = (value: string) =>
  props.options.find((option) => option.value === value)?.label || value;
</script>

<template>
  <div class="w">
    <div class="select" :class="!collapsed && 'expanded'" ref="select" @click="toggle">
      <span class="value" :class="!value && 'placeholder'">
        <Icon
          v-if="value && getOptionIcon(value)"
          :name=" getOptionIcon(value) as string"
          :size="16"
        />
        {{ getOptionLabel(value) || placeholder || "Choose one" }}
      </span>
      <div class="arrowIcon">
        <ChevronDown />
      </div>
    </div>
    <div v-if="!collapsed" class="dropdown">
      <div
        class="option"
        :class="value === option.value && 'selectedOption'"
        v-for="option in options"
        @click="value = option.value"
      >
        <Icon v-if="option.icon" :name="option.icon" :size="16" />
        {{ option.label || option.value }}
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.w {
  position: relative;
  width: fit-content;
}

.select {
  display: inline-flex;
  align-items: center;
  user-select: none;
  position: relative;
  border: 1px solid var(--accent-2-color);
  border-radius: 6px;
  background-color: var(--background-color);
  transition: border 150ms ease-in, color 200ms ease-out, box-shadow 200ms ease;
  --select-font-size: calc(0.75 * 16px);
  font-size: var(--select-font-size);
  height: calc(2.25 * 16px);
  min-width: 11.5em;
  padding: 0 calc(0.334 * 16px) 0 calc(0.667 * 16px);
  cursor: pointer;

  &:hover,
  :global(.expanded) {
    border-color: var(--foreground-color) !important;
  }
}

:global(.placeholder) {
  font-size: calc(0.875 * 16px) !important;
  color: var(--accent-3-color) !important;
}

.value {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  height: 100%;
  margin-right: 1.25em;
  line-height: 1;
  width: calc(100% - 1.25em);
  color: var(--foreground-color);
}

.arrowIcon {
  position: absolute;
  right: 4pt;
  font-size: var(--select-font-size);
  top: 50%;
  bottom: 0;
  transform: translateY(-50%) rotate(v-bind(iconRotate));
  pointer-events: none;
  transition: transform 200ms ease;
  display: flex;
  align-items: center;
  width: 17px;
  height: 17px;

  svg {
    color: var(--accent-5-color) !important;
  }
}

.dropdown {
  box-sizing: border-box;
  border: 1px solid var(--accent-2-color);
  background-color: var(--background-color);
  border-radius: 6px;
  position: absolute;
  top: 37px;
  left: 0;
  width: 100%;
  overflow-y: auto;
  overflow-anchor: none;
  padding: 0.38em 0;
  scroll-behavior: smooth;
  z-index: 1100;

  .option {
    display: flex;
    align-items: center;
    gap: 6px;
    max-width: 100%;
    box-sizing: border-box;
    color: var(--accent-5-color);
    cursor: pointer;
    transition: background 0.2s ease 0s, border-color 0.2s ease 0s;
    --select-font-size: calc(0.75 * 16px);
    font-size: var(--select-font-size);
    height: calc(2.25 * 16px);
    padding: 0 calc(0.667 * 16px) 0 calc(0.667 * 16px);
    margin: 0 0 0 0;

    &:hover {
      background-color: var(--accent-1-color);
      color: var(--accent-7-color);
    }

    svg {
      color: inherit !important;
    }
  }

  :global(.selectedOption) {
    background-color: var(--accent-2-color);
    color: var(--foreground-color) !important;
  }
}
</style>
