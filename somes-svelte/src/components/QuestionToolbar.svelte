<script lang="ts">

</script>

<div class="flex flex-col gap-[1rem]">
    <div class="flex flex-row h-18 gap-[2vw]">
        <div class="flex flex-row h-[2.5rem] mt-auto gap-[0.5vw]">
            <input
                class="input rounded-lg p-2"
                type="text"
                placeholder="Suchbegriff eingeben"
            />
            <button type="button" class="btn variant-filled">Suchen</button>
        </div>	
        <label class="label ml-auto w-1/6">
            <span>Sortieren nach</span>
            <select class="select">
                <option value="1">Relevanz</option>
                <option value="2">Benutzerwertung</option>
                <option value="3">Neueste</option>
            </select>
        </label>
    </div>
    <div class="flex flex-row h-18 gap-[2vw]">
        <label class="label">
            <span>Delegate</span>
            <select class="select">
                <option value="1">Option 1</option>
                <option value="2">Option 2</option>
                <option value="3">Option 3</option>
                <option value="4">Option 4</option>
                <option value="5">Option 5</option>
            </select>
        </label>
        <label class="label">
            <span>Party</span>
            <select class="select">
                <option value="1">Option 1</option>
                <option value="2">Option 2</option>
                <option value="3">Option 3</option>
                <option value="4">Option 4</option>
                <option value="5">Option 5</option>
            </select>
        </label>
        <label class="label">
            <span>Date Range</span>
            <div class="flex flex-row h-10 gap-[0.5vw]">
                <input class="input rounded-lg p-2" title="Input (date)" type="date" />
                <div class="mt-2">
                    to
                </div>
                <input class="input rounded-lg p-2" title="Input (date)" type="date" />
            </div>
        </label>
    </div>
</div>
<hr class="opacity-100 mt-auto" />