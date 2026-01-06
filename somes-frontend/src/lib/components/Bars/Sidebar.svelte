<script lang="ts">
    import homeIcon from '$lib/assets/icons/home.svg?raw';
    import delegatesIcon from '$lib/assets/icons/delegates.svg?raw';
    import statisticsIcon from '$lib/assets/icons/statistics.svg?raw';
    import somesIcon from '$lib/assets/somes_icon.svg?raw';
	import userIcon from '$lib/assets/icons/user.svg?raw';
	import { page } from '$app/state';

	let activeUrl = $state(page.url.pathname);
    let isSelected: (href: string) => boolean = () => false;
    import { resolve } from '$app/paths';
	import VoteParliament2 from '../Parliaments/VoteParliament2.svelte';
	import { mockDelegatesNoColor, mockVoteResult } from '$lib/parliaments/mock';

	$effect(() => {
		activeUrl = page.url.pathname;
        isSelected = (href: string) => {
            console.log(href);
            return activeUrl?.includes(href);
        };
	});

</script>

<div class="w-19 flex flex-col items-center justify-center gap-4 h-screen bg-surface-500">
    <a 
        href="{resolve("/home")}" 
        title="Somes" 
        class="hover:cursor-pointer stroke-white fill-white rounded-xl h-9 w-9 mt-4 flex justify-center items-center"
    >
        <span class="w-7 flex items-center justify-center stroke-white! fill-white ">
            {@html somesIcon}
        </span>
    </a>
    <a 
        href="{resolve("/home")}" 
        title="Neuigkeiten" 
        class="{activeUrl?.includes("/home") ? 'bg-tertiary-500 stroke-black' : ' stroke-white'} hover:cursor-pointer rounded-xl h-9 w-9 flex justify-center items-center"
    >
        <span class="w-6">
            {@html homeIcon}
        </span>
    </a>
    <a 
        href="{resolve("/history")}" 
        title="Abstimmungshistorie"
        class="{activeUrl?.includes("/history") ? 'bg-tertiary-500 stroke-black' : ' stroke-white'} hover:cursor-pointer rounded-xl h-9 w-9 flex justify-center items-center"
    >
        <span class="w-9">
            <VoteParliament2
					againstOpacity={0.3}
					voteResult={mockVoteResult()}
					delegates={mockDelegatesNoColor()}
					preview
					overrideDelegates
					noSeats
					useOffset={false}
					enforceSvg
                    showGovs={false}
                    forceColor={activeUrl?.includes("/history") ? 'black' : 'white'}
				/>
        </span>
    </a>
    <a 
        href="{resolve("/delegates")}" 
        title="Abgeordnete" 
        class="{activeUrl?.includes("/delegates") ? 'bg-tertiary-500 fill-black' : ' fill-white'} hover:cursor-pointer rounded-xl h-9 w-9 flex justify-center items-center"
    >
        <span class="w-5">
            {@html delegatesIcon}
        </span>
    </a>
    <a 
        href="{resolve("/statistics")}" 
        title="Statistiken" 
        class="{activeUrl?.includes("/statistics") ? 'bg-tertiary-500 fill-black' : ' fill-white'} hover:cursor-pointer rounded-xl h-9 w-9 flex justify-center items-center"
    >
        <span class="w-5">
            {@html statisticsIcon}
        </span>
    </a>

    <div class="flex flex-col mt-auto mb-4 gap-3">
        <!-- <DarkMode class="text-primary-500 dark:text-primary-600 border dark:border-gray-800 hover:bg-primary-800" /> -->
        <a 
            href="{resolve("/user")}" 
            title="Benutzerprofil" 
            class="{activeUrl?.includes("/user") ? 'bg-tertiary-500 fill-black' : ' fill-white'} hover:cursor-pointer rounded-xl h-9 w-9 flex justify-center items-center"
        >
            <span class="w-5">
                {@html userIcon}
            </span>
        </a>

    </div>
</div>
