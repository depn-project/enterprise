<script setup lang="ts">
import { Option } from "./model";
import { ref, watch } from "vue";
import Icon from "@/shared/ui/Icon";

interface Props {
  options: Array<Option>;
  modelValue?: string;
}

interface Emits {
  (e: "update:modelValue", payload: string): void;
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

const value = ref<string>(props.modelValue || props.options[0].value);

watch(value, (value) => emit("update:modelValue", value));
</script>

<template>
  <div class="switch" role="radiogroup">
    <button
      v-for="option in options"
      :aria-checked="option.value === value"
      role="radio"
      @click="value = option.value"
    >
      <span>
        <Icon v-if="option.icon" :name="option.icon" :size="16" />
        {{ option.name }}
      </span>
    </button>
  </div>
</template>

<scoped scoped lang="scss">
.switch {
  display: flex;
  align-items: center;
  position: relative;
  width: fit-content;
  padding: 3px;
  background-color: var(--background-color);
  border: 1px solid var(--accent-2-color);
  border-radius: var(--radius);
  height: var(--form-height);
  font-size: var(--form-font);
  user-select: none;

  button {
    margin: 0;
    font-size: inherit;
    height: 100%;
    cursor: pointer;
    background: var(--background-color);
    padding: 0;
    border-radius: 3px;
    font-weight: 500;
    border: none;
    outline-color: var(--foreground-color);
    color: var(--accent-4-color);
    transition-property: color;
    transition-duration: 0.15s;

    &:hover {
      color: var(--foreground-color);
    }

    &[aria-checked="true"] {
      color: var(--foreground-color);
      background-color: var(--accent-2-color);
    }

    span {
      display: flex;
      justify-content: center;
      align-items: center;
      gap: 6px;
      padding: 0 12px;
    }
  }
}
</scoped>
