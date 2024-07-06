<script lang="ts">
	import { TabAnchor, TabGroup } from "@skeletonlabs/skeleton";
	import { page } from '$app/stores';
	import homeIcon from "$lib/assets/icons/home.svg?raw";
	import userIcon from "$lib/assets/icons/user.svg?raw";
	import statsIcon from "$lib/assets/icons/statistics.svg?raw";

	$: isSelected = (href: string) => $page.url.pathname?.includes(href);
</script>

<TabGroup 
    justify="justify-center"
    active="variant-filled-primary"
    hover="hover:variant-soft-primary"
    flex="flex-1 lg:flex-none"
    rounded=""
    border=""
    class="bg-surface-100-800-token w-full"
>
    <TabAnchor href="/home" selected={isSelected("/home")}>
        <div class="flex justify-center flex-col items-center">
            <div>
                {@html homeIcon}
            </div>
            <div>
                Neuigkeiten
            </div>
        </div>
    </TabAnchor>
    <TabAnchor href="/statistics" selected={isSelected("/statistics")}>
        <div class="flex justify-center flex-col items-center">
            <div class="w-8">
                {@html statsIcon}
            </div>
            <div>
                Statistiken
            </div>
        </div>
    </TabAnchor>
    <TabAnchor href="/user" selected={isSelected("/user")}>
        <div class="flex justify-center flex-col items-center">
            <div class="">
                {@html userIcon}
            </div>
            <div>
                Benutzer
            </div>
        </div>
    </TabAnchor>
</TabGroup>
