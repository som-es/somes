<script lang="ts">
	import { login } from '$lib/api/register';

    // import { login } from '$lib/api';
	import { drawerStore } from '@skeletonlabs/skeleton';
	// import Login from 'svelte-google-materialdesign-icons/Login.svelte';

    let username = "";
    let pwd = "";
</script>

<section class="login-wrapper">
    <section class="login-container">
        <h1>Login</h1>
        <form>
            <label class="label">
                <span>Benutzername</span>
                <div class="input-group input-group-divider grid-cols-[1fr_auto]">
                    <input class="input" type="text" placeholder="example" bind:value={username}/>
                </div>
            </label>
            <label class="label">
                <span>Password</span>
                <div class="input-group input-group-divider grid-cols-[1fr_auto]">
                    <input class="input" type="password" placeholder="password" bind:value={pwd}/>
                </div>
            </label>
            <button type="button" class="btn variant-filled" on:click={async () => {
                if (await login(username, pwd)) {
                    drawerStore.close();
                }
            }}>
                <!-- <span><Login /></span> -->
                <span>Anmelden</span>
            </button>
        </form>
        <span>Noch kein Account? Registriere dich <a href="./register.html">HIER!</a></span>
    </section>
</section>

<style lang="scss">
    .login-wrapper {
        width: 100%;
	    height: 100vh;
	    display: flex;
        justify-content: center;
	    align-items: center;

        .login-container {
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