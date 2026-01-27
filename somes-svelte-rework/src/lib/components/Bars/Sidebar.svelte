<script lang="ts">
	import { page } from '$app/stores';
	import { base } from '$app/paths';
	import somesIconBlue from '$lib/assets/icons_sidebar/somes_icon_blue_bg.png';
	import parliamentIcon from '$lib/assets/icons_sidebar/parlament_logo.svg?raw';
	import InfinityIcon from '$lib/assets/icons_sidebar/infinity.svg?raw';
	import PeopleIcon from '$lib/assets/icons_sidebar/people.svg?raw';
	import StatisticsIcon from '$lib/assets/icons_sidebar/statistics.svg?raw';

	type SidebarItem = {
		label: string;
		href: string;
		icon: string;
		containerClass?: string;
		innerClass?: string;
	};

	const items: SidebarItem[] = [
		{
			label: 'Neuigkeiten',
			href: `${base}/home`,
			icon: InfinityIcon,
			containerClass: 'mb-3 mt-3',
			innerClass: 'w-6'
		},
		{
			label: 'Abstimmungen',
			href: `${base}/history/votes`,
			icon: parliamentIcon,
			containerClass: 'my-2',
			innerClass: 'w-6 flex items-center justify-center',
		},
		{
			label: 'Regierung',
			href: `${base}/delegates`,
			icon: PeopleIcon,
			containerClass: 'my-3',
			innerClass: 'w-5'
		},
		{
			label: 'Statistik',
			href: `${base}/statistics`,
			icon: StatisticsIcon,
			containerClass: 'my-3',
			innerClass: 'w-5'
		}
	];

	$: isActive = (href: string) => $page.url.pathname.includes(href);
</script>

<div class="h-full bg-surface-500 w-20 flex flex-col items-center">
	<a href="{base}/" class="block">
		<img src={somesIconBlue} alt="Somes Icon" class="w-14 my-8" />
	</a>

	{#each items as item}
		{@const active = isActive(item.href)}
		<a
			href={item.href}
			class="group relative flex flex-col items-center {item.containerClass}"
		>
			<div
				class="w-10 h-10 rounded-2xl flex items-center justify-center z-10 transition-colors duration-200 {active ? 'bg-[#F8DFC9]' : ''}"
			>
				<div class="{item.innerClass} {active ? 'text-black' : 'text-white'}">
					{@html item.icon}
				</div>
			</div>
			
			<span
				class="absolute opacity-0 text-[10px] text-white font-medium whitespace-nowrap pointer-events-none z-0 transition-all duration-300
				{!active ? 'group-hover:opacity-100 group-hover:translate-y-7' : ''}"
			>
				{item.label}
			</span>
		</a>
	{/each}
</div>
