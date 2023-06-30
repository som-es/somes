<script lang="ts">
	import { goto } from '$app/navigation';
    import ParliamentImg from '$lib/assets/assets/parliament.png';
	import { localStorageStore, type DrawerSettings, drawerStore } from '@skeletonlabs/skeleton';
	import { get, type Readable, type Writable } from 'svelte/store';


    const noAccountStorage: Readable<boolean | null> = localStorageStore('noAccount', null);
    const isNoAccount = get(noAccountStorage);

    if (isNoAccount) {
        goto("/home");
    }

    function redirectToHome() {
        console.log("redirecting to home");
        const noAccountStorage: Writable<boolean | null> = localStorageStore('noAccount', null);
        noAccountStorage.set(true);
        // localStorageStore('noAccount', true);
        goto("/home");
    }

    const loginDrawerSettings: DrawerSettings = {
		id: "login-drawer",
		position: "right",
		width: "w-[280px] md:w-[480px]",
	}

    drawerStore.open(loginDrawerSettings);

</script>

<div class="h-full w-full background">
    <div class="trans-layer">
        <!-- <div class="flex h-full flex-row flex-wrap"> -->
            <!-- <div class="w-2/3"> -->
                <!-- Test -->
            <!-- </div> -->
            <!-- <div class="w-1/3 bg-tertiary-200"> -->
                <!-- Test -->
            <!-- </div> -->
        <!-- </div> -->
        <div class="container mx-auto px-4 self-center sm:text-left"> 
            <h2 class="text-tertiary-300 font-bold pt-28 text-center sm:text-left">Experience Democracy!</h2>
            
            <div class="flex justify-center sm:justify-start pt-6 self-center sm:pl-6">            
                <button class="text-center bg-tertiary-500 text-white rounded-full px-14 h-9">Log In</button>
                <button on:click="{_ => redirectToHome()}" class="ml-4 text-center bg-secondary-400 text-white rounded-full px-15 h-9">Continue without Account</button>
            </div>
            <div class="mt-2">
                <h5><span class="text-tertiary-100 font-semibold">Don't have an account?</span> <a href="/register" class="!text-secondary-100 font-bold">Sign up!</a></h5>
            </div>
        </div>
    </div>
</div>

<style>
.background {
    background-image: url("$lib/assets/parliament.png");
    background-repeat: no-repeat;
    background-attachment: fixed;
    background-position: center;
    background-size: cover;
    /* background-color: rgba(46, 54, 68, 0.63); */
    /* box-shadow: inset 0 0 0 1000px rgba(46, 54, 68, 0.63); */
}    

.trans-layer {
    background-color:rgba(46, 54, 68, 0.63); 
    width: 100%;
    height: 100%;
}

a:link{
  text-decoration: none!important;

}
</style>