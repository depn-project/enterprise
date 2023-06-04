<script lang="ts">
  import "carbon-components-svelte/css/all.css";

  import {
    Theme,
    RadioButtonGroup,
    RadioButton,
  } from "carbon-components-svelte";
  import type { CarbonTheme } from "carbon-components-svelte/types/Theme/Theme.svelte";
  import { onMount } from "svelte";
  import Router, { push } from "svelte-spa-router";
  import Setup from "@/pages/Setup";
  import Servers from "@/pages/Servers";
  import Server from "@/pages/Server";
  import Settings from "@/pages/Settings";
  import Header from "@/widgets/Header";
  import "@/shared/lib/i18n";
  import axios from "axios";

  let theme: CarbonTheme = "white";

  onMount(async () => {
    const ip = localStorage.getItem("ip");
    const port = localStorage.getItem("port");
    const username = localStorage.getItem("username");
    const password = localStorage.getItem("password");

    try {
      await axios.get(`http://${ip}:${port}/`, {
        auth: { username, password },
      });
      push("/servers");
    } catch {
      push("/");
    }
  });
</script>

<Header />
<Theme bind:theme />
<Router
  routes={{
    "/": Setup,
    "/servers": Servers,
    "/server/:id": Server,
    "/settings": Settings,
  }}
/>

<style scoped>
  :global(#app) {
    height: 100vh;
  }
</style>
