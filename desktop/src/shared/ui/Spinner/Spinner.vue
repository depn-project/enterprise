<script setup lang="ts">
interface Props {
  size?: number;
}

withDefaults(defineProps<Props>(), {
  size: 16,
});
</script>

<template>
  <div class="spinner">
    <div class="container">
      <span v-for="blade in 12" :key="blade" class="blade"></span>
    </div>
  </div>
</template>

<style scoped lang="scss">
.spinner {
  display: block;
  box-sizing: border-box;
  width: calc(1.25px * v-bind(size));
  height: calc(1.25px * v-bind(size));
  padding: 0;
  margin: 0;

  .container {
    width: 100%;
    height: 100%;
    position: relative;
    left: 50%;
    top: 50%;
  }

  .blade {
    position: absolute;
    background-color: var(--spinner-blade-color);
    top: -3.9%;
    width: 24%;
    height: 8%;
    left: -10%;
    border-radius: 6px;
    animation: spinner 1.2s linear 0s infinite normal none running;

    @for $i from 1 through 12 {
      &:nth-child(#{$i}) {
        animation-delay: -(1.3 - 0.1 * $i + s);
        transform: rotate((30 * $i) + deg) translate(146%);
      }
    }
  }
}

@keyframes spinner {
  0% {
    opacity: 1;
  }

  100% {
    opacity: 0.15;
  }
}
</style>
