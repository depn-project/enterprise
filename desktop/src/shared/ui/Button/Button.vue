<script setup lang="ts">
import { computed, ref } from "vue";
import { useDark } from "@vueuse/core";
import { ButtonType } from "./model";
import ButtonDrip from "./ButtonDrip.vue";

interface Props {
  type: ButtonType;
  ghost: boolean;
  loading: boolean;
  disabled: boolean;
  icon?: string;
  iconRight?: string;
  label?: string;
  onClick?: () => void;
}

const props = withDefaults(defineProps<Props>(), {
  type: "default",
  ghost: false,
  loading: false,
  disabled: false,
});

const buttonRef = ref<HTMLButtonElement | null>(null);
const isDark = useDark();
const dripShow = ref<boolean>(false);
const dripX = ref<number>(0);
const dripY = ref<number>(0);
const dripColor = computed(() =>
  isDark.value ? "rgba(51, 51, 51, 0.65)" : "rgba(234, 234, 234, 0.65)"
);

const dripCompletedHandle = () => {
  dripShow.value = false;
  dripX.value = 0;
  dripY.value = 0;
};

const clickHandler = (event: MouseEvent) => {
  if (props.disabled || props.loading) return;

  const showDrip = !props.ghost;

  if (showDrip && buttonRef.value) {
    const rect = buttonRef.value.getBoundingClientRect();
    dripShow.value = true;
    dripX.value = event.clientX - rect.left;
    dripY.value = event.clientY - rect.top;
  }

  props.onClick && props.onClick();
};
</script>

<template>
  <button @click="clickHandler" ref="buttonRef">
    <div class="text">{{ label }}</div>
    <ButtonDrip
      v-if="dripShow"
      :x="dripX"
      :y="dripY"
      :color="dripColor"
      :onCompleted="dripCompletedHandle"
    />
  </button>
</template>

<style scoped lang="scss">
button {
  display: inline-block;
  line-height: calc(2.5 * 16px);
  border-radius: 6px;
  font-weight: 400;
  font-size: calc(0.875 * 16px);
  user-select: none;
  outline: none;
  text-transform: capitalize;
  justify-content: center;
  text-align: center;
  white-space: nowrap;
  transition: background-color 200ms ease 0ms, box-shadow 200ms ease 0ms, border 200ms ease 0ms,
    color 200ms ease 0ms;
  position: relative;
  overflow: hidden;
  color: var(--accent-5-color);
  background-color: var(--background-color);
  border: 1px solid var(--accent-3-color);
  cursor: pointer;
  box-shadow: none;
  min-width: calc(10.5 * 16px);
  width: initial;
  height: calc(2.5 * 16px);
  padding: 0 calc(1.375 * 16px) 0 calc(1.375 * 16px);
  margin: 0;

  &:hover,
  &:focus {
    color: var(--foreground-color);
    background-color: var(--background-color);
    border-color: var(--foreground-color);
  }
}

.text {
  position: relative;
  z-index: 1;
}
</style>
