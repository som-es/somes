<script lang="ts">
	import { register } from "$lib/api/register";
	import { drawerStore, type PopupSettings } from "@skeletonlabs/skeleton";
	import { verificationMailStore } from "../../stores/stores";
	import { loginDrawerSettings } from "$lib/constants";
	import { t } from "$lib/translations";

	/*$: if ($bearer != null) {
        window.location.replace("./");
    }*/

	let username = "";
	let email = "";
	let pwd = "";
	let pwdVerify = "";
	$: userErrorMessage = username === "" ? $t("common.user_empty") : "";
	$: emailErrorMessage = email === "" ? $t("common.email_empty") : "";
	$: pwdErrorMessage =
		pwd !== pwdVerify
			? $t("common.password_mismatch")
			: pwd === "" || pwdVerify === ""
			? $t("common.password_empty")
			: "";

	let pwdPopupSettings: PopupSettings = {
		event: "hover",
		target: "pwdErrorPopup",
	};
	let userPopupSettings: PopupSettings = {
		event: "hover",
		target: "userErrorPopup",
	};

	let waitForVerification = false;
</script>

{#if !waitForVerification}
	<div class="login_container flex flex-col text-black">
		<h2 class="text-center">{$t("common.sign_up")}</h2>
		<label for="username">{$t("common.username")}</label>
		<span class="text-red-500">{userErrorMessage}</span>
		<input
			id="username"
			class="down"
			placeholder="mustermann21"
			type="text"
			bind:value={username}
		/>

		<label for="email">{$t("common.email")}</label>
		<span class="text-red-500">{emailErrorMessage}</span>
		<input
			id="email"
			class="down"
			placeholder="max.mustermann@gmail.com"
			type="text"
			bind:value={email}
		/>

		<label for="password">{$t("common.password")}</label>
		<input id="password" class="down" type="password" bind:value={pwd} />

		<label for="passwordVerify">{$t("common.password_repeat")}</label>
		<input id="passwordVerify" type="password" bind:value={pwdVerify} />
		<span class="down text-red-500">{pwdErrorMessage}</span>

		<input
			type="button"
			value={$t("common.sign_up_action")}
			on:click={async () => {
				if (email === "" || pwd === "" || username === "") {
					return;
				}

				let res = await register(username, email, pwd);
				if (res == null) {
					waitForVerification = true;
					verificationMailStore.set(email);
					return;
				}
				if (res.username_taken) {
					userErrorMessage = $t("common.user_taken");
				}
				if (res.email_taken) {
					emailErrorMessage = $t("common.email_taken");
				}

				if (res.invalid_email) {
					emailErrorMessage = $t("common.email_invalid");
				}

				if (res.insufficient_password) {
					pwdErrorMessage = $t("common.password_criteria");
					// TODO: definieren, grafisch darstellen..
				}
			}}
		/>
	</div>
{:else}
	<div class="login_container flex flex-col">
		<h2 class="text-center">{$t("sign_up_verify")}</h2>
		<h4 class="text-center mt-2">
			{$t("verification_to")} <span class="font-bold text-green-400">{email}</span>
			{$t("common.sent")}.
		</h4>
		<p class="mt-1 text-center">
			{$t("sign_up_check_email")}
		</p>
	</div>
	<div class="login_container">
		<p class="text-center">{$t("sign_up_login_after")}</p>
		<input
			on:click={(_) => drawerStore.open(loginDrawerSettings)}
			class="w-full mt-1"
			type="button"
			value={$t("common.login")}
		/>
	</div>
{/if}

<style>
	.down {
		margin-bottom: 20px;
	}

	input[type="text"],
	input[type="password"] {
		padding: 10px;
		border: none;
		border-radius: 5px;
		box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.2);
		font-size: 16px;
	}

	.login_container {
		background-color: #fff;
		margin: 20px auto;
		width: 600px;
		max-width: 95%;
		padding: 20px;
		border-radius: 10px;
		box-shadow: 0px 0px 20px rgba(0, 0, 0, 0.2);
	}

	input[type="button"] {
		/* background-color: #5c5cd6; */
		background-color: rgb(var(--color-tertiary-500));
		color: #fff;
		padding: 10px;
		border: none;
		border-radius: 5px;
		box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.2);
		font-size: 20px;
		cursor: pointer;
		transition: all 0.3s ease;
	}

	input[type="button"]:hover {
		background-color: #fff;
		color: rgb(var(--color-tertiary-500));
	}
</style>
