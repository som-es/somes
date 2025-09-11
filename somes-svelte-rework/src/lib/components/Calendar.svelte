<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	interface Day {
		name: string;
		enabled: boolean;
		date: Date;
        item: Item | null;
	}

	interface Item {
		title: string;
		className: string;
		date: Date;
		len: number;
		startRow: number;
		startCol: number;
		detailHeader: string | null;
		detailContent: string | null;
		isBottom: boolean;
	}

	export let headers: string[] = []; // should only have 5 headers now (Mon–Fri)
	export let days: Day[] = [];
	export let items: Item[] = [];

	const dispatch = createEventDispatcher();

	let days2D: Day[][] = [];

	$: {
		days2D = [];
		let acc: Day[] = [];
		days.forEach((day, i) => {
			if (i > 0 && i % 5 === 0) {
				days2D.push([...acc]);
				acc = [];
			}
			acc.push(day);
		});
		if (acc.length > 0) {
            for (let i = 0; i < 4 - acc.length; i++) {
                days2D.push()
            }
            days2D.push(acc);
        }
	}
</script>

<div class="calendar max-w-[50rem]">
	<div class="flex flex-col">
		<div class="flex flex-row">
			{#each headers as header}
				<span class="day-name text-secondary-500" on:click={() => dispatch('headerClick', header)}>
					{header}
				</span>
			{/each}
		</div>

		{#each days2D as week}
			<div class="flex flex-row">
                {#each Array(5) as _, i} 
                    {#if week[i]}
                        {#if week[i].enabled}
                            <div class="flex-1 flex items-center">
                                <span class="badge text-xs bg-tertiary-300">Plenarsitzung</span>
                                <div class="day">{week[i].name}</div>
                            </div>
                        {:else}
                            <div class="day day-disabled">{week[i].name}</div>
                        {/if}
                    {:else}
                        <div class="flex-1">
                            <div class="day empty-day h-full"></div>
                        </div>
                    {/if}
                {/each}
			</div>
		{/each}
	</div>
</div>

<style>
.calendar {
  overflow: auto;
  display: flex;
  flex-direction: column;
}

.flex-row {
  display: flex;
  flex-direction: row;
}

.day {
  border-bottom: 1px solid rgba(166, 168, 179, 0.12);
  border-right: 1px solid rgba(166, 168, 179, 0.12);
  text-align: right;
  padding: 14px 20px;
  font-size: 14px;
  color: #98a0a6;
  flex: 1; /* evenly distribute 5 days per row */
  box-sizing: border-box;
}

.day-name {
  font-size: 12px;
  text-transform: uppercase;
  text-align: center;
  border-bottom: 1px solid rgba(166, 168, 179, 0.12);
  line-height: 50px;
  font-weight: 500;
  flex: 1; /* evenly distribute headers */
}

.day-disabled {
  color: rgba(152, 160, 166, 0.5);
  background-color: #fff;
  cursor: not-allowed;
}

.task {
  border-left-width: 3px;
  padding: 8px 12px;
  margin: 10px;
  border-left-style: solid;
  font-size: 14px;
  position: relative;
  align-self: center;
  z-index: 2;
  border-radius: 15px;
}
</style>
