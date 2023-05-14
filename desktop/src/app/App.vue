<script setup lang="ts">
import ThemeSwitcher from "@/features/ThemeSwitcher";
import Button from "@/shared/ui/Button";
import { RpcClient } from "jsonrpc-ts";
import { computed, ref } from "vue";

const connected = ref<boolean>(false);
const label = computed(() => (connected.value ? "Deactivate" : "Activate"));
const header = computed(() => (connected.value ? "connected" : "disconnected"));
const loading = ref<boolean>(false);

interface VPNRpc {
  "vpn.connect": () => string;
  "vpn.disconnect": () => string;
}

const rpcClient = new RpcClient<VPNRpc>({ url: "http://127.0.0.1:3030" });

const click = async () => {
  loading.value = true;

  await rpcClient.makeRequest({
    jsonrpc: "2.0",
    method: connected.value ? "vpn.disconnect" : "vpn.connect",
    id: 1,
  });

  connected.value = !connected.value;

  loading.value = false;
};
</script>

<template>
  <ThemeSwitcher />
  <h1 style="color: var(--foreground-color)">Status: {{ header }}</h1>
  <Button @click="click" :label="label" />
</template>

<style scoped></style>
