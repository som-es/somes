<script lang='ts'>
	// The ordering of these imports is critical to your app working properly
	// import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	import '../theme.postcss';

	// If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
	import '@skeletonlabs/skeleton/styles/all.css';
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';
	import { AppShell, Drawer, LightSwitch, drawerStore } from '@skeletonlabs/skeleton';
	import LoginDrawer from './LoginDrawer.svelte';
	import { userStore } from '../stores/stores';
	import { get } from 'svelte/store';

	const parliamentUrl = new URL('$lib/assets/somes-browner.png', import.meta.url).href
	drawerStore.close();

	// $: user = get(userStore);

	const profileLink = "/profile";
	// $: if (user == null) {
		// profileLink = "/";
	// }

</script>

<Drawer>
	{#if $drawerStore.id === 'login-drawer'}
		<LoginDrawer />
    {/if}
</Drawer>

<!-- App Shell -->
<AppShell>
	<section class="bg-primary-800 demo flex justify-between items-center px-4">
		<!-- <img class="mx-4 w-1/6" src="{parliamentUrl}"/> -->
		<!-- <div style = "width: 100%; height: 20px"> -->
	
		<img class="mx-4" style="width: 24vh;" src="{parliamentUrl}" alt="somes logo"/>
		<div>
			<LightSwitch />
		</div>
		<!-- </div> -->
	</section>
	<nav class="bg-primary-500 navbar">
		<a href="">
			<div>
				<svg xmlns="http://www.w3.org/2000/svg" width="30" height="30" viewBox="0 0 30 30">
					<path id="Pfad_9" data-name="Pfad 9" d="M4.113,18.111a3.365,3.365,0,0,0-.352-3.27,7.449,7.449,0,0,1-1.533-4.529,7.5,7.5,0,1,1,7.5,7.5A7.61,7.61,0,0,1,7.957,17.6a1.851,1.851,0,0,0-1.439.246q-.288.185-.591.352a13.452,13.452,0,0,1-2.339,1.055c.131-.27.253-.533.37-.8.052-.111.1-.229.15-.346Zm-4.134-7.8a10.646,10.646,0,0,0,2.151,6.463c-.042.1-.089.205-.131.3A16.807,16.807,0,0,1,.284,20.127a1.684,1.684,0,0,0-.216,1.518A1.154,1.154,0,0,0,1.1,22.5a12.082,12.082,0,0,0,5.751-1.74c.225-.129.45-.264.666-.4a9.155,9.155,0,0,0,2.208.264,10.042,10.042,0,0,0,9.75-10.312A10.042,10.042,0,0,0,9.729,0,10.042,10.042,0,0,0-.021,10.313ZM20.228,28.125a9.487,9.487,0,0,0,2.208-.264c.216.135.441.27.666.4A12.082,12.082,0,0,0,28.853,30a1.146,1.146,0,0,0,1.031-.85,1.665,1.665,0,0,0-.216-1.518,17.387,17.387,0,0,1-1.716-3.053c-.042-.1-.089-.2-.131-.3a10.65,10.65,0,0,0,2.156-6.469,10.076,10.076,0,0,0-9.29-10.3,13.274,13.274,0,0,1,.291,2.8v.035a7.546,7.546,0,0,1,6.75,7.465A7.42,7.42,0,0,1,26.2,22.336a3.367,3.367,0,0,0-.352,3.27c.052.117.1.234.15.346.117.264.244.527.37.8a13.029,13.029,0,0,1-2.339-1.055q-.3-.167-.591-.352A1.851,1.851,0,0,0,22,25.1a7.473,7.473,0,0,1-8.184-3.439,10.821,10.821,0,0,1-2.344.691A9.726,9.726,0,0,0,20.228,28.125Z" transform="translate(0.022)" fill="#fff"/>
				</svg>                  
			</div>
		</a>
		<a href="/home">          
			<div>
			<svg xmlns="http://www.w3.org/2000/svg" width="81.416" height="30" viewBox="0 0 81.416 28.318">
				<g id="somes_logo" transform="translate(-384.929 -308.973)">
				<path id="Pfad_2" data-name="Pfad 2" d="M422.49,329.818a38.077,38.077,0,0,1-10.008,4.39c-3.626,1-8.4,2.317-14.1.755-2.281-.633-6.685-1.831-9.434-6.082a10.571,10.571,0,0,1-1.979-6.846,10.117,10.117,0,0,1,3.49-6.5,12.188,12.188,0,0,1,5.212-2.594,20.635,20.635,0,0,1,7.077-.807,25.083,25.083,0,0,1,7.719,1.779,36.3,36.3,0,0,1,6.1,2.915,46.626,46.626,0,0,1,4.328,2.915c1.337.98,1.375,1.1,3.928,3.08,2.447,1.9,2.447,1.839,3.8,2.924.166.13.317.252.461.373" transform="translate(0 -0.3)" fill="none" stroke="#fff" stroke-linecap="round" stroke-miterlimit="10" stroke-width="4"/>
				<path id="Pfad_3" data-name="Pfad 3" d="M439.9,316.745a38.078,38.078,0,0,1,10.008-4.39c3.626-1,8.4-2.317,14.1-.755,2.281.633,6.685,1.831,9.434,6.082a10.571,10.571,0,0,1,1.979,6.846,10.117,10.117,0,0,1-3.49,6.5,12.187,12.187,0,0,1-5.212,2.594,20.633,20.633,0,0,1-7.077.807,25.084,25.084,0,0,1-7.719-1.779,36.3,36.3,0,0,1-6.1-2.915,46.619,46.619,0,0,1-4.328-2.915c-1.337-.98-1.375-1.1-3.928-3.08-2.447-1.9-2.447-1.839-3.8-2.924-.166-.13-.317-.252-.461-.373" transform="translate(-11.112 0)" fill="none" stroke="#fff" stroke-linecap="round" stroke-miterlimit="10" stroke-width="4"/>
				</g>
			</svg>
			</div>
		</a>
		
		<a href={profileLink}>
			<svg xmlns="http://www.w3.org/2000/svg" width="30" height="30" viewBox="0 0 30 30">
				<path id="Pfad_8" data-name="Pfad 8" d="M15,15c4.734,0,8.571-3.358,8.571-7.5S19.734,0,15,0,6.429,3.358,6.429,7.5,10.266,15,15,15Zm-3.06,2.812C5.344,17.812,0,22.488,0,28.26A1.877,1.877,0,0,0,1.989,30H28.011A1.877,1.877,0,0,0,30,28.26c0-5.771-5.344-10.447-11.94-10.447Z" fill="#fff"/>
			</svg>                  
		</a>
	</nav>
	<!--<AppBar>-->		
		<!-- <button class="float-right2">Toggle Theme</button> -->
		
	<!--</AppBar>-->
	<!-- This is where your app content will go -->
	<slot />

</AppShell>

<style>
.float-right2 {
  
}

.demo {
	
}

.navbar {
	position: sticky;
	top: 0;
	z-index: 9;
	display: flex;
	justify-content: space-around;
	padding: 10px;
}

</style>