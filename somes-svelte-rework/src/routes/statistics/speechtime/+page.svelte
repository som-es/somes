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

    const delegateSimpleSpeechTime = async (gp: string | null, gender: string | null, isDesc: boolean | true): Promise<DelegateData[]> => {
        const response = errorToNull(await justPost<DelegateSpeechTime[]>('speechtime_per_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
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

  const partySimpleSpeechTime = async (gp: string | null, gender: string | null, isDesc: boolean | true): Promise<DelegateData[]> => {
      const response = errorToNull(await justPost<DelegateSpeechTime[]>('speechtime_per_party', {
          legis_period: gp,
          party: null,
          gender,
          is_desc: isDesc
      })) ?? [];

      console.log(response)

      return response.map(val => {
          return {
              name: null,
              party: val.delegate_party,
              data: val.total_speech_time
          };
      });
  }

</script>

<Container>
    <DelegateBarChartControl height={3100} id={0} delegateMakeRequest={delegateSimpleSpeechTime} />
</Container>

<Container>
    <DelegateBarChartControl height={400} id={1} delegateMakeRequest={partySimpleSpeechTime} />
</Container>

