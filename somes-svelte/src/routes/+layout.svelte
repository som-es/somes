<script lang="ts">
	// Global CSS imports
	import "../theme.postcss";
	import "@skeletonlabs/skeleton/styles/all.css";
	import "../app.postcss";
	import '@fortawesome/fontawesome-svg-core/styles.css'

	// Popup imports
	import { computePosition, autoUpdate, offset, shift, flip, arrow } from "@floating-ui/dom";
	import { Modal, storePopup, type ModalComponent } from "@skeletonlabs/skeleton";
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	import { AppBar, AppShell, Drawer, LightSwitch, drawerStore } from "@skeletonlabs/skeleton";
	import LoginDrawer from "@/components/LoginDrawer.svelte";
	import { userStore } from "../stores/stores";
	import { get } from "svelte/store";
	import { config } from '@fortawesome/fontawesome-svg-core'
	import QuestionModal from "@/components/QuestionModal.svelte";
	import LangSwitch from "@/components/LangSwitch.svelte";

  config.autoAddCss = false;

	const parliamentUrl = new URL("$lib/assets/somes_with_text.svg", import.meta.url).href;
	drawerStore.close();

	// $: user = get(userStore);

	const profileLink = "/profile";
	// $: if (user == null) {
	// profileLink = "/";
	// }

	const modalComponentRegistry: Record<string, ModalComponent> = {
		QuestionModal: {
			ref: QuestionModal,
			slot: "<p>Skeleton</p>",
		},
	};
</script>

<Drawer>
	{#if $drawerStore.id === "login-drawer"}
		<LoginDrawer />
	{/if}
</Drawer>

<Modal components={modalComponentRegistry} />

<AppShell>
	<!-- TODO: show the second header below the the first while also making it sticky (without the first one being sticky) -->
	<svelte:fragment slot="header">
		<AppBar background="bg-surface-50-900-token" padding="p-2">
			<svelte:fragment slot="lead">
				<a class="mx-4" href="/">
					<svg
						class="fill-current stroke-current"
						xmlns="http://www.w3.org/2000/svg"
						width="16vh"
						viewBox="0 0 2244.417 622.549"
					>
						<g id="Gruppe_76" data-name="Gruppe 76" transform="translate(7925.417 -13505)">
							<g id="Logo" transform="translate(-7910.417 13566.545)">
								<path
									id="Pfad_1"
									data-name="Pfad 1"
									d="M853.355,224.34,830.6,254.505H411.945L389.19,224.34Z"
									transform="translate(-378.445 -224.34)"
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-miterlimit="10"
									stroke-width="30"
								/>
								<path
									id="Pfad_2"
									data-name="Pfad 2"
									d="M610.587,409.082a258.4,258.4,0,0,1-62.943,24.037c-22.8,5.463-52.825,12.684-88.691,4.133-14.346-3.468-42.041-10.023-59.333-33.3-4.275-5.653-14.394-19.334-12.446-37.481,2.185-20,17.529-32.113,21.947-35.581,10.5-8.313,20.855-11.021,32.778-14.2,4.038-1.14,21.186-5.463,44.511-4.418,21,1,36.293,5.843,48.549,9.738A244.2,244.2,0,0,1,573.3,337.968a299.127,299.127,0,0,1,27.22,15.961c8.408,5.368,8.646,6.033,24.7,16.864,15.391,10.4,15.391,10.071,23.895,16.009,1.045.713,2,1.378,2.9,2.043"
									transform="translate(-386.928 104.821)"
									fill="none"
									stroke-linecap="round"
									stroke-miterlimit="10"
									stroke-width="30"
								/>
								<path
									id="Pfad_3"
									data-name="Pfad 3"
									d="M474.733,342.993a258.439,258.439,0,0,1,62.944-24.038c22.8-5.463,52.825-12.683,88.691-4.133,14.346,3.468,42.041,10.023,59.332,33.3,4.276,5.653,14.395,19.334,12.446,37.482-2.185,20-17.529,32.113-21.947,35.581-10.5,8.313-20.854,11.021-32.778,14.2-4.038,1.14-21.187,5.463-44.512,4.418-21-1-36.293-5.843-48.55-9.738a244.3,244.3,0,0,1-38.336-15.961A299.016,299.016,0,0,1,484.8,398.146c-8.408-5.368-8.646-6.033-24.7-16.864-15.391-10.4-15.391-10.071-23.894-16.009-1.046-.713-2-1.378-2.9-2.042"
									transform="translate(-212.976 100.223)"
									fill="none"
									stroke-linecap="round"
									stroke-miterlimit="10"
									stroke-width="30"
								/>
								<path
									id="Pfad_4"
									data-name="Pfad 4"
									d="M447.387,237.82a539.326,539.326,0,0,1,9.073,93.2,513.657,513.657,0,0,1-3.9,69.879c-4.275,32.493-16.2,90.685-55.675,160.517"
									transform="translate(-349.567 -173.784)"
									fill="none"
									stroke-linecap="round"
									stroke-miterlimit="10"
									stroke-width="30"
								/>
								<path
									id="Pfad_5"
									data-name="Pfad 5"
									d="M476.39,237.82a539.283,539.283,0,0,0-9.074,93.2,513.542,513.542,0,0,0,3.9,69.879c4.276,32.493,16.2,90.685,55.675,160.517"
									transform="translate(-85.563 -173.784)"
									fill="none"
									stroke-linecap="round"
									stroke-miterlimit="10"
									stroke-width="30"
								/>
								<line
									id="Linie_1"
									data-name="Linie 1"
									y2="304.028"
									transform="translate(244.324 68.644)"
									fill="none"
									stroke-linecap="round"
									stroke-miterlimit="10"
									stroke-width="30"
								/>
							</g>
							<text
								id="somes_"
								data-name="somes"
								transform="translate(-7313 13505)"
								font-size="500"
								font-family="ProximaSoft-Regular, Proxima Soft"
								><tspan x="0" y="460">somes</tspan></text
							>
						</g>
					</svg>
				</a>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<LangSwitch />
				<LightSwitch width="w-12" />
			</svelte:fragment>
		</AppBar>
		<AppBar
			gridColumns="grid-cols-3"
			slotDefault="place-self-center"
			slotTrail="place-content-end"
			padding="p-2"
		>
			<svelte:fragment slot="lead">
				<a href="/questions" class="pl-[25vw]">
					<div>
						<svg
							class="fill-current"
							xmlns="http://www.w3.org/2000/svg"
							width="30"
							height="30"
							viewBox="0 0 30 30"
						>
							<path
								id="Pfad_9"
								data-name="Pfad 9"
								d="M4.113,18.111a3.365,3.365,0,0,0-.352-3.27,7.449,7.449,0,0,1-1.533-4.529,7.5,7.5,0,1,1,7.5,7.5A7.61,7.61,0,0,1,7.957,17.6a1.851,1.851,0,0,0-1.439.246q-.288.185-.591.352a13.452,13.452,0,0,1-2.339,1.055c.131-.27.253-.533.37-.8.052-.111.1-.229.15-.346Zm-4.134-7.8a10.646,10.646,0,0,0,2.151,6.463c-.042.1-.089.205-.131.3A16.807,16.807,0,0,1,.284,20.127a1.684,1.684,0,0,0-.216,1.518A1.154,1.154,0,0,0,1.1,22.5a12.082,12.082,0,0,0,5.751-1.74c.225-.129.45-.264.666-.4a9.155,9.155,0,0,0,2.208.264,10.042,10.042,0,0,0,9.75-10.312A10.042,10.042,0,0,0,9.729,0,10.042,10.042,0,0,0-.021,10.313ZM20.228,28.125a9.487,9.487,0,0,0,2.208-.264c.216.135.441.27.666.4A12.082,12.082,0,0,0,28.853,30a1.146,1.146,0,0,0,1.031-.85,1.665,1.665,0,0,0-.216-1.518,17.387,17.387,0,0,1-1.716-3.053c-.042-.1-.089-.2-.131-.3a10.65,10.65,0,0,0,2.156-6.469,10.076,10.076,0,0,0-9.29-10.3,13.274,13.274,0,0,1,.291,2.8v.035a7.546,7.546,0,0,1,6.75,7.465A7.42,7.42,0,0,1,26.2,22.336a3.367,3.367,0,0,0-.352,3.27c.052.117.1.234.15.346.117.264.244.527.37.8a13.029,13.029,0,0,1-2.339-1.055q-.3-.167-.591-.352A1.851,1.851,0,0,0,22,25.1a7.473,7.473,0,0,1-8.184-3.439,10.821,10.821,0,0,1-2.344.691A9.726,9.726,0,0,0,20.228,28.125Z"
								transform="translate(0.022)"
							/>
						</svg>
					</div>
				</a>
			</svelte:fragment>
			<a href="/home">
				<div>
					<svg
						class="stroke-current"
						xmlns="http://www.w3.org/2000/svg"
						width="81.416"
						height="30"
						viewBox="0 0 81.416 28.318"
					>
						<g id="somes_logo" transform="translate(-384.929 -308.973)">
							<path
								id="Pfad_2"
								data-name="Pfad 2"
								d="M422.49,329.818a38.077,38.077,0,0,1-10.008,4.39c-3.626,1-8.4,2.317-14.1.755-2.281-.633-6.685-1.831-9.434-6.082a10.571,10.571,0,0,1-1.979-6.846,10.117,10.117,0,0,1,3.49-6.5,12.188,12.188,0,0,1,5.212-2.594,20.635,20.635,0,0,1,7.077-.807,25.083,25.083,0,0,1,7.719,1.779,36.3,36.3,0,0,1,6.1,2.915,46.626,46.626,0,0,1,4.328,2.915c1.337.98,1.375,1.1,3.928,3.08,2.447,1.9,2.447,1.839,3.8,2.924.166.13.317.252.461.373"
								transform="translate(0 -0.3)"
								fill="none"
								stroke-linecap="round"
								stroke-miterlimit="10"
								stroke-width="4"
							/>
							<path
								id="Pfad_3"
								data-name="Pfad 3"
								d="M439.9,316.745a38.078,38.078,0,0,1,10.008-4.39c3.626-1,8.4-2.317,14.1-.755,2.281.633,6.685,1.831,9.434,6.082a10.571,10.571,0,0,1,1.979,6.846,10.117,10.117,0,0,1-3.49,6.5,12.187,12.187,0,0,1-5.212,2.594,20.633,20.633,0,0,1-7.077.807,25.084,25.084,0,0,1-7.719-1.779,36.3,36.3,0,0,1-6.1-2.915,46.619,46.619,0,0,1-4.328-2.915c-1.337-.98-1.375-1.1-3.928-3.08-2.447-1.9-2.447-1.839-3.8-2.924-.166-.13-.317-.252-.461-.373"
								transform="translate(-11.112 0)"
								fill="none"
								stroke-linecap="round"
								stroke-miterlimit="10"
								stroke-width="4"
							/>
						</g>
					</svg>
				</div>
			</a>
			<svelte:fragment slot="trail">
				<a href={profileLink} class="pr-[25vw]">
					<svg
						class="fill-current"
						xmlns="http://www.w3.org/2000/svg"
						width="30"
						height="30"
						viewBox="0 0 30 30"
					>
						<path
							id="Pfad_8"
							data-name="Pfad 8"
							d="M15,15c4.734,0,8.571-3.358,8.571-7.5S19.734,0,15,0,6.429,3.358,6.429,7.5,10.266,15,15,15Zm-3.06,2.812C5.344,17.812,0,22.488,0,28.26A1.877,1.877,0,0,0,1.989,30H28.011A1.877,1.877,0,0,0,30,28.26c0-5.771-5.344-10.447-11.94-10.447Z"
						/>
					</svg>
				</a>
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<slot />
</AppShell>
