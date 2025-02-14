<script lang="ts">
	import { errorToNull, justPost } from "$lib/api/api";
	import Container from "$lib/components/Layout/Container.svelte";
	import DelegateBarChartControl from "$lib/components/Statistics/DelegateBarChartControl.svelte";
	import type { DelegateData } from "$lib/types";

    
    type DelegateSpeechTime = {
		delegate_name: string;
    delegate_gender: string;
		delegate_party: string;
		total_speech_time: number;
	};

  let tableHeightDelegate = 0;
  let tableHeightParty= 0;
  let tableHeightGender = 0;
  let tableHeightAge = 0;

  const calculateTableHeight = (numRows: number): number => {
      const maxHeight = 3100;
      const maxRows = 100;

      const height = (numRows / maxRows) * maxHeight;

      return Math.min(height, maxHeight);
  };

    const delegateSimpleSpeechTime = async (gp: string | null, gender: string | null, isDesc: boolean | true): Promise<DelegateData[]> => {
        const response = errorToNull(await justPost<DelegateSpeechTime[]>('speechtime_per_delegate', {
			legis_period: gp,
			party: null,
			gender,
			is_desc: isDesc
		})) ?? [];

        console.log(response)

        tableHeightDelegate = calculateTableHeight(response.length);

        console.log(tableHeightDelegate)

        return response.map(val => {
            return {
                name: val.delegate_name,
                gender: null,
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

      // tableheight berechnung ist die gefehrlich, da response hier 14 (Zahl der Parteien) zurückgibt
      console.log(response)

      tableHeightParty = calculateTableHeight(response.length);

      console.log(tableHeightParty)

      return response.map(val => {
          return {
              name: null,
              gender: null,
              party: val.delegate_party,
              data: val.total_speech_time
          };
      });
  }

  const genderSimpleSpeechTime = async (gp: string | null, gender: string | null, isDesc: boolean | true): Promise<DelegateData[]> => {
      const response = errorToNull(await justPost<DelegateSpeechTime[]>('speechtime_per_gender', {
          legis_period: gp,
          party: null,
          gender,
          is_desc: isDesc
      })) ?? [];

      //console.log(response)

      tableHeightGender = calculateTableHeight(response.length);

      console.log(tableHeightGender)

      return response.map(val => {
          return {
              name: null,
              gender: val.delegate_gender,
              party: null,
              data: val.total_speech_time
          };
      });
  }

  const ageSimpleSpeechTime = async (gp: string | null, gender: string | null, isDesc: boolean | true): Promise<DelegateData[]> => {
      const response = errorToNull(await justPost<DelegateSpeechTime[]>('speechtime_per_age', {
          legis_period: gp,
          party: null,
          gender,
          is_desc: isDesc
      })) ?? [];

      //console.log(response)

      tableHeightAge = calculateTableHeight(response.length);

      console.log(tableHeightAge)

      return response.map(val => {
          return {
              name: val.delegate_name,
              gender: null,
              party: val.delegate_party,
              data: val.total_speech_time
          };
      });
  }




</script>

<Container>
    <DelegateBarChartControl height={tableHeightDelegate} id={0} delegateMakeRequest={delegateSimpleSpeechTime} title="Redezeit pro Abgeordneten (in Mintuen)"/>
</Container>

<Container>
    <DelegateBarChartControl height={tableHeightParty} id={1} delegateMakeRequest={partySimpleSpeechTime} title="Redezeit nach Parteien (in Minuten)" />
</Container>

<Container>
    <DelegateBarChartControl height={tableHeightGender} id={2} delegateMakeRequest={genderSimpleSpeechTime} title="Redezeit nach Geschlecht (in Minuten)"/>
</Container>

<Container>
    <DelegateBarChartControl height={tableHeightAge} id={3} delegateMakeRequest={ageSimpleSpeechTime} title="Redezeit nach Alter (in Minuten)"/>
</Container>