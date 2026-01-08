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
	import { getSeats } from '$lib/caching/seats';

	$effect(() => {
		activeUrl = page.url.pathname;
        isSelected = (href: string) => {
            return activeUrl?.includes(href);
        };
	});

    const submenu = [
		/*{
			title: 'Statistiken',
			route: '/statistics',
			list: [
				{ href: resolve(`/statistics/overview`), label: 'Übersicht', keywords: '' },
				{ href: resolve(`/statistics/age`), label: 'Alter', keywords: '' },
				{
					href: resolve(`/statistics/political_positions`),
					label: 'Politische Positionen',
					keywords: ''
				}
			]
		},
		{
			title: 'Redebezogen',
			route: '/statistics',
			list: [
				{ href: resolve(`/statistics/speechtime`), label: 'Redezeit', keywords: '' },
				{ href: resolve(`/statistics/total_speeches`), label: 'Gehaltene Reden', keywords: '' },
				{ href: resolve(`/statistics/speechcomplexity`), label: 'Sprachkomplexität', keywords: '' }
			]
		},
		{
			title: 'Aktivitäten',
			route: '/statistics',
			list: [
				{ href: resolve(`/statistics/absences`), label: 'Abwesenheiten', keywords: '' },
				{ href: resolve(`/statistics/activity`), label: 'Aktivität', keywords: '' },
				{
					href: resolve(`/statistics/division_accuracy`),
					label: 'Bereichssprechergenauigkeit',
					keywords: ''
				},
				{ href: resolve(`/statistics/call_to_orders`), label: 'Ordnungsrufe', keywords: '' }
			]
		},
		{
			title: 'Abstimmungen',
			route: '/statistics',
			list: [
				{
					href: resolve(`/statistics/absolute_majority_initiatives`),
					label: '2/3 Abstimmugnen',
					keywords: ''
				},
				{
					href: resolve(`/statistics/votes_together`),
					label: 'Zusammenabstimmen der Parteien',
					keywords: ''
				}
			]
		},*/

		{
			title: 'Nationalrat',
			route: '/history',
			list: [
				{ href: resolve(`/history/votes`), label: 'Abstimmungen', keywords: '' },
				{ href: resolve(`/history/unfinished_votes`), label: 'Zur Abstimmung', keywords: '' }
			]
		},
		{
			title: 'Regierung',
			route: '/history',
			list: [
				{ href: resolve(`/history/proposals`), label: 'Ministerialentwürfe', keywords: '' },
				{ href: resolve(`/history/decrees`), label: 'Verordnungen', keywords: '' }
			]
		}
	];

</script>

<div class="h-full bg-surface-50 grid-cols-[auto_1fr] flex lg:grid">
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
        href="{resolve("/history/votes")}" 
        title="Abstimmungshistorie"
        class="{activeUrl?.includes("/history") ? 'bg-tertiary-500 stroke-black' : ' stroke-white'} hover:cursor-pointer rounded-xl h-9 w-9 flex justify-center items-center"
    >
        <span class="w-9">
            <VoteParliament2
					againstOpacity={0.3}
					voteResult={mockVoteResult()}
					delegates={mockDelegatesNoColor()}
                    allSeats={new Map([["XX", getSeats(new Map(), "XX", true)]])}
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

{#if activeUrl?.includes('/statistics') || activeUrl?.includes('/history')}
    <section class="p-3 text-white pb-20 space-y-4 overflow-y-auto bg-primary-500 max-w-60">
        {#each submenu as segment, i}
            {#if activeUrl?.includes(segment.route)}
                <!-- Title -->
                <p class="font-bold pl-4 text-2xl">{segment.title}</p>
                <!-- Nav List -->
                <nav class="list-nav">
                    <ul class="my-1">
                        {#each segment.list as { href, label }}
                            <li class="p-2">
                                <a
                                    {href}
                                    class="p-2 rounded-3xl {activeUrl?.includes(href) ? 'bg-secondary-500 text-black' : 'hover:bg-secondary-300'}"
                                    data-sveltekit-preload-data="hover"
                                >
                                    <span class="flex-auto">{@html label}</span>
                                    <!-- {#if badge}<span class="badge variant-filled-secondary">{badge}</span>{/if} -->
                                </a>
                            </li>
                        {/each}
                    </ul>
                </nav>
                <!-- Divider -->
                {#if i + 1 < submenu.length}<hr class="my-6! opacity-50" />{/if}
            {/if}
        {/each}
    </section>
{/if}
</div>