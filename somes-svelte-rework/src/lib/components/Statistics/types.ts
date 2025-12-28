
export type Config ={
		
		filter_info_1:FilterInfoStatistics | null;
		filter_info_2:FilterInfoStatistics | null;
		filter_info_3:FilterInfoStatistics | null;
		filter_info_4:FilterInfoStatistics | null;

	};

export type FilterInfoStatistics = {
	name:string, 
	isShown:boolean, 
	infoText:string | null
}