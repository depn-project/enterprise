<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import axios from "axios";
  import {
    Button,
    ButtonSet,
    Form,
    InlineLoading,
    InlineNotification,
    PasswordInput,
    TextInput,
  } from "carbon-components-svelte";
  import { ArrowRight } from "carbon-icons-svelte";
  import { field, form } from "svelte-forms";
  import { required } from "svelte-forms/validators";
  import { _ } from "svelte-i18n";

  export let serverId: number;

  const username = field("username", "", [required()]);
  const password = field("password", "", [required()]);

  const authForm = form(username, password);

  let loading = false;
  let error = false;
  let connected = false;
  let errorText = "";

  const handleClickSecondary = () => {
    authForm.clear();
    error = false;
  };

  const disconnect = async () => {
    await invoke("disconnect");
    connected = false;
    authForm.clear();
  };

  const submit = async (e: SubmitEvent) => {
    e.preventDefault();
    await authForm.validate();

    if ($authForm.errors.length) return;
    loading = true;
    error = false;

    const ip = localStorage.getItem("ip");
    const port = localStorage.getItem("port");
    const usernameStorage = localStorage.getItem("username");
    const passwordStorage = localStorage.getItem("password");

    if (
      usernameStorage !== $username.value ||
      passwordStorage !== $password.value
    ) {
      error = true;
      loading = false;
      errorText = $_("feature.auth.error.invalid");
      return;
    }

    try {
      const res = await axios.get(`http://${ip}:${port}/server/${serverId}`, {
        auth: { username: $username.value, password: $password.value },
      });

      await invoke("create_config", { config: res.data });
      await invoke("connect");
      connected = true;

      loading = false;
    } catch (e) {
      console.log(e);
      loading = false;
    }
  };
</script>

<div class="auth-form">
  <Form on:submit={submit}>
    <div class="fields-container">
      {#if error}
        <InlineNotification
          class="error"
          title={$_("carbon.inline-notification.error")}
          subtitle={errorText}
        />
      {/if}
      <div class="inputs">
        <TextInput
          invalid={$authForm.hasError("username.required")}
          invalidText={$_("feature.setup.error.username.required")}
          labelText={$_("feature.setup.label.username")}
          disabled={connected}
          bind:value={$username.value}
        />
        <PasswordInput
          invalid={$authForm.hasError("password.required")}
          invalidText={$_("feature.setup.error.password.required")}
          labelText={$_("feature.setup.label.password")}
          disabled={connected}
          bind:value={$password.value}
        />
      </div>
    </div>
    <div class="bx--row">
      <div class="bx--col">
        <div class="button-container">
          <ButtonSet>
            <Button
              kind="secondary"
              on:click={handleClickSecondary}
              disabled={connected}
            >
              {$_("feature.auth.button.clear")}
            </Button>
            {#if loading}
              <InlineLoading
                class="loading"
                status="active"
                description={$_("shared.loading")}
              />
            {:else if connected}
              <Button icon={ArrowRight} kind="danger" on:click={disconnect}>
                {$_("feature.auth.button.disconnect")}
              </Button>
            {:else}
              <Button type="submit" icon={ArrowRight}>
                {$_("feature.auth.button.connect")}
              </Button>
            {/if}
          </ButtonSet>
        </div>
      </div>
    </div>
  </Form>
</div>

<style scoped lang="scss">
  .auth-form {
    width: 424px;

    form {
      display: flex;
      flex-direction: column;
    }

    .fields-container {
      padding: 1rem;
      padding-top: 0;

      :global(.bx--label) {
        font-size: 0.875rem;
      }
    }

    :global(.bx--inline-notification) {
      margin-top: 0;
      max-width: none;
    }

    .inputs {
      display: flex;
      flex-direction: column;
      gap: 8px;
    }

    .button-container {
      display: flex;
      padding: 1rem;
      padding-top: 0;

      :global(.bx--inline-loading) {
        padding-left: 16px;
      }
    }
  }
</style>
