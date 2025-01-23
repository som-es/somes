<script lang="ts">
	import { errorToNull, justPost } from "$lib/api/api";
	import Container from "$lib/components/Layout/Container.svelte";
	import DelegateBarChartControl from "$lib/components/Statistics/DelegateBarChartControl.svelte";
	import type { DelegateData } from "$lib/types";

    
    type DelegateSpeechTime = {
		delegate_name: string;
		delegate_party: string;
		total_speech_time: number;
	};

    const simpleSpeechTime = async (gp: string | null, gender: string | null): Promise<DelegateData[]> => {
        const response = errorToNull(await justPost<DelegateSpeechTime[]>('speechtime_per_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: true
		})) ?? [];

        console.log(response)

        return response.map(val => {
            return {
                name: val.delegate_name,
                party: val.delegate_party,
                data: val.total_speech_time
            };
        });
    }

</script>

<Container>
    <DelegateBarChartControl makeRequest={simpleSpeechTime} />
</Container>