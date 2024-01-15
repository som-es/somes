<script lang="ts">
	import { base } from "$app/paths";
	// import { bearer } from '../../stores/store';
	//import loginIcon from '../../lib/images/login24x200.svg?raw';

	import { popup } from "@skeletonlabs/skeleton";
	import type { PopupSettings } from "@skeletonlabs/skeleton";
	//import { register } from '../../lib/api';

	let username = "";
	let pwd = "";
	let pwdVerify = "";
	let nameTaken = false;
	$: userError = username === "" || nameTaken;
	$: userErrorMessage =
		username === "" ? "Benutzername darf nicht leer sein!" : "Benutzername existiert schon!";
	$: pwdError = pwd !== pwdVerify || pwd === "" || pwdVerify === "";
	$: pwdErrorMessage =
		pwd !== pwdVerify
			? "Passwörter stimmen nicht überein!"
			: pwd === "" || pwdVerify === ""
			? "Passwort darf nicht leer sein"
			: "";

	let pwdPopupSettings: PopupSettings = {
		event: "hover",
		target: "pwdErrorPopup",
	};
	let userPopupSettings: PopupSettings = {
		event: "hover",
		target: "userErrorPopup",
	};
</script>

<section class="register-wrapper">
	<section class="register-container">
		<h1>Registrierung</h1>
		<form>
			<label class="label">
				<span>Benutzername</span>
				<div class="input-group input-group-divider grid-cols-[1fr_auto]">
					<input type="text" placeholder="example" bind:value={username} />
					{#if userError}
						<div class="input-group-shim" use:popup={userPopupSettings}>
							<span class="badge-icon variant-filled-warning"> ! </span>
						</div>
					{/if}
				</div>
			</label>
			<label class="label">
				<span>Password</span>
				<div class="input-group input-group-divider grid-cols-[1fr_auto]">
					<input class="input" type="password" placeholder="password" bind:value={pwd} />
					{#if pwdError}
						<div class="input-group-shim" use:popup={pwdPopupSettings}>
							<span class="badge-icon variant-filled-warning"> ! </span>
						</div>
					{/if}
				</div>
			</label>
			<label class="label">
				<span>Password bestätigen</span>
				<div class="input-group input-group-divider grid-cols-[1fr_auto]">
					<input class="input" type="password" placeholder="password" bind:value={pwdVerify} />
					{#if pwdError}
						<div class="input-group-shim" use:popup={pwdPopupSettings}>
							<span class="badge-icon variant-filled-warning"> ! </span>
						</div>
					{/if}
				</div>
			</label>
			<button type="button" class="btn variant-filled" on:click={async () => {}}>
				<span>Registrieren</span>
			</button>
		</form>
		<span>Du hast schon einen Account? Ab zum <a href="{base}/">LOGIN!</a></span>
	</section>
</section>
<div class="card w-auto shadow-xl py-2 px-4" data-popup="pwdErrorPopup">
	<div class="arrow bg-surface-100-800-token" />
	<span>{pwdErrorMessage}</span>
</div>
<div class="card w-auto shadow-xl py-2 px-4" data-popup="userErrorPopup">
	<div class="arrow bg-surface-100-800-token" />
	<span>{userErrorMessage}</span>
</div>

<style lang="scss">
	.register-wrapper {
		width: 100%;
		height: 100vh;
		display: flex;
		justify-content: center;
		align-items: center;

		.register-container {
			display: flex;
			flex-direction: column;
			justify-content: center;
			align-items: center;

			h1 {
				margin-bottom: 50px;
			}

			form {
				display: flex;
				flex-direction: column;
				justify-content: center;
				width: 120%;

				& > * {
					margin-bottom: 20px;
					width: 100%;
				}

				& > button {
					margin-top: 20px;
				}
			}
		}
	}
</style>
