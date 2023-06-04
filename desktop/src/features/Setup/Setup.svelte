<script lang="ts">
  import {
    TextInput,
    PasswordInput,
    Form,
    Button,
    InlineLoading,
    ButtonSet,
    ToastNotification,
  } from "carbon-components-svelte";
  import ArrowRight from "carbon-icons-svelte/lib/ArrowRight.svelte";
  import { _ } from "svelte-i18n";

  import { form, field } from "svelte-forms";
  import { required } from "svelte-forms/validators";
  import { ipv4, port as portValidator } from "@/shared/lib/forms/validators";
  import axios from "axios";
  import { push } from "svelte-spa-router";

  const ip = field("ip", "", [required(), ipv4()]);
  const port = field("port", "", [required(), portValidator()]);
  const username = field("username", "", [required()]);
  const password = field("password", "", [required()]);

  $: ipInvalidText =
    ($setupForm.hasError("ip.required") &&
      $_("feature.setup.error.ip.required")) ||
    ($setupForm.hasError("ip.invalid_ip_address") &&
      $_("feature.setup.error.ip.invalid"));

  $: portInvalidText =
    ($setupForm.hasError("port.required") &&
      $_("feature.setup.error.port.required")) ||
    ($setupForm.hasError("port.invalid_port") &&
      $_("feature.setup.error.port.invalid"));

  const setupForm = form(ip, port, username, password);

  let loading = false;
  let error = false;

  const handleClickSecondary = () => {
    setupForm.clear();
    error = false;
  };

  const submit = async (e: SubmitEvent) => {
    e.preventDefault();
    await setupForm.validate();

    if ($setupForm.errors.length) return;
    loading = true;

    try {
      await axios.get(`http://${$ip.value}:${$port.value}/`, {
        auth: {
          username: $username.value,
          password: $password.value,
        },
      });

      localStorage.setItem("ip", $ip.value);
      localStorage.setItem("port", $port.value);
      localStorage.setItem("username", $username.value);
      localStorage.setItem("password", $password.value);

      push("/servers");
    } catch (e) {
      error = true;
    }

    loading = false;
  };
</script>

<div class="setup-form">
  <Form on:submit={submit}>
    <div class="fields-container">
      <div class="heading-container">
        <div class="bx--row">
          <div class="bx--col">
            <div class="heading">
              {$_("feature.setup.header")}
            </div>
          </div>
        </div>
      </div>
      {#if error}
        <ToastNotification
          class="error"
          fullWidth
          title={$_("carbon.inline-notification.error")}
          subtitle={$_("feature.setup.error.form")}
        />
      {/if}
      <div class="inputs">
        <TextInput
          invalid={$setupForm.hasError("ip.required") ||
            $setupForm.hasError("ip.invalid_ip_address")}
          invalidText={ipInvalidText}
          labelText={$_("feature.setup.label.ip")}
          bind:value={$ip.value}
        />
        <TextInput
          invalid={$setupForm.hasError("port.required") ||
            $setupForm.hasError("port.invalid_port")}
          invalidText={portInvalidText}
          labelText={$_("feature.setup.label.port")}
          bind:value={$port.value}
        />
        <TextInput
          invalid={$setupForm.hasError("username.required")}
          invalidText={$_("feature.setup.error.username.required")}
          labelText={$_("feature.setup.label.username")}
          bind:value={$username.value}
        />
        <PasswordInput
          invalid={$setupForm.hasError("password.required")}
          invalidText={$_("feature.setup.error.password.required")}
          labelText={$_("feature.setup.label.password")}
          bind:value={$password.value}
        />
      </div>
    </div>
    <div class="bx--row">
      <div class="bx--col">
        <div class="button-container">
          <ButtonSet>
            <Button kind="secondary" on:click={handleClickSecondary}>
              {$_("feature.setup.button.clear")}
            </Button>
            {#if loading}
              <InlineLoading
                class="loading"
                status="active"
                description={$_("shared.loading")}
              />
            {:else}
              <Button type="submit" icon={ArrowRight}>
                {$_("feature.setup.button.sign-in")}
              </Button>
            {/if}
          </ButtonSet>
        </div>
      </div>
    </div>
  </Form>
</div>

<style scoped lang="scss">
  .setup-form {
    width: 424px;
    background-color: var(--cds-ui-background);

    .fields-container {
      padding: 1rem;
      padding-top: 0;

      :global(.bx--label) {
        font-size: 0.875rem;
      }

      .heading-container {
        margin-top: 0.5rem;
        padding-bottom: 1rem;

        .heading {
          font-size: 1.75rem;
          font-weight: 400;
          line-height: 1.28572;
          letter-spacing: 0;
        }
      }
    }

    .error {
      width: 424px;
    }

    .inputs {
      display: flex;
      flex-direction: column;
      gap: 8px;
    }

    .button-container {
      display: flex;
      padding: 1rem;

      :global(.bx--inline-loading) {
        padding-left: 16px;
      }
    }
  }
</style>
