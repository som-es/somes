<script lang="ts">
	import { goto } from '$app/navigation';
    import ParliamentImg from '$lib/assets/assets/parliament.png';
	import { localStorageStore } from '@skeletonlabs/skeleton';
	import { redirect } from '@sveltejs/kit';
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

</script>

<div class="h-full w-full background">

    <div class="container mx-auto px-4 self-center sm:text-left"> 
        <h2 class="text-tertiary-300 font-bold pt-28 text-center sm:text-left">Experience Democracy!</h2>
        
        <div class="flex justify-center sm:justify-start pt-6 self-center sm:pl-6">            
            <button class="text-center bg-tertiary-500 text-white rounded-full px-14 h-9">Log In</button>
            <button on:click="{_ => redirectToHome()}" class="ml-4 text-center bg-secondary-400 text-white rounded-full px-15 h-9">Continue without Account</button>
        </div>
        <div class="mt-2 text-center sm:text-left">
            <span class="text-tertiary-100 font-semibold">Don't have an account?</span> <a href="#top" class="!text-secondary-600 font-bold">Sign up!</a>
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
    box-shadow: inset 0 0 0 1000px rgba(46, 54, 68, 0.63);
}    
a:link{
  text-decoration: none!important;

}
</style>