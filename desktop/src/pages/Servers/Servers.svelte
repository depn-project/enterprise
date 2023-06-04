<script lang="ts">
  import axios from "axios";
  import { _ } from "svelte-i18n";
  import { push } from "svelte-spa-router";
  import { onMount } from "svelte";
  import {
    DataTable,
    Dropdown,
    Link,
    Select,
    SelectItem,
  } from "carbon-components-svelte";
  import { locationI18n } from "@/shared/lib/location";
  import "/node_modules/flag-icons/css/flag-icons.min.css";
  import Auth from "@/features/Auth/Auth.svelte";

  let servers = [];
  let selected = 0;

  $: selectedServer = servers.find((server) => server.id === selected);

  onMount(async () => {
    const ip = localStorage.getItem("ip");
    const port = localStorage.getItem("port");
    const username = localStorage.getItem("username");
    const password = localStorage.getItem("password");

    try {
      const res = await axios.get(`http://${ip}:${port}/servers`, {
        auth: { username, password },
      });

      servers = res.data.map((server) => ({
        ...server,
        location: locationI18n(server.country, server.city, $_),
      }));

      selected = servers[0].id;
    } catch {
      push("/");
    }
  });
</script>

<div class="servers-page">
  <div class="w">
    <h1>{$_("page.servers.title")}</h1>
    <Dropdown
      titleText={$_("page.servers.select")}
      selectedId={selected}
      items={servers.map((server) => ({ ...server, text: server.location }))}
    />
    <div class="ip">
      <span class="address">{selectedServer?.ip}</span>
      <span class="fi fi-ru" />
    </div>
  </div>
  <Auth serverId={selected} />
</div>

<style scoped lang="scss">
  .servers-page {
    .w {
      padding: 1rem;
    }

    :global(.bx--label) {
      font-size: 0.875rem;
    }

    h1 {
      padding-bottom: 1rem;
    }

    .ip {
      padding: 8px 0;

      .address {
        margin-right: 4px;
      }

      .fi {
        border: 1px solid black;
        background-size: cover;
      }
    }
  }
</style>
