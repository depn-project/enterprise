<script setup lang="ts">
import { computed, onMounted, onBeforeUnmount, ref } from "vue";

interface Props {
  x: number;
  y: number;
  onCompleted: () => void;
  color: string;
}

const props = withDefaults(defineProps<Props>(), {
  x: 0,
  y: 0,
});

const dripRef = ref<HTMLDivElement | null>();
const top = computed(() => (Number.isNaN(+props.y) ? 0 : props.y - 10));
const left = computed(() => (Number.isNaN(+props.x) ? 0 : props.x - 10));

onMounted(() => {
  if (!dripRef.value) return;
  dripRef.value.addEventListener("animationend", props.onCompleted);
});

onBeforeUnmount(() => {
  if (!dripRef.value) return;
  dripRef.value.removeEventListener("animationend", props.onCompleted);
});
</script>

<template>
  <div ref="dripRef" class="drip">
    <svg width="20" height="20" viewBox="0 0 20 20" :style="`top: ${top}; left: ${left}`">
      <g stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
        <g :fill="color">
          <rect width="100%" height="100%" rx="10" />
        </g>
      </g>
    </svg>
  </div>
</template>

<style scoped lang="scss">
.drip {
  position: absolute;
  inset: 0;

  svg {
    position: absolute;
    animation: 350ms ease-in expand;
    animation-fill-mode: forwards;
    width: 1rem;
    height: 1rem;
  }

  @keyframes expand {
    0% {
      opacity: 0;
      transform: scale(1);
    }
    30% {
      opacity: 1;
    }
    80% {
      opacity: 0.5;
    }
    100% {
      transform: scale(28);
      opacity: 0;
    }
  }
}
</style>
