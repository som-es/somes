import { departments_per_gp } from "$lib/api/api";
import { decrees_by_search } from "$lib/components/Delegates/Decrees/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ fetch, setHeaders, url }) => {
    if (process.env.NODE_ENV === 'production') {
        setHeaders({
            'cache-control': 'max-age=120'
        });
    }
    
    const searchParams = url.searchParams;
    if (searchParams.get("page") == null) {
        searchParams.set("page", "1")
    }
    const queryParams = searchParams.toString();
    const filter = `${queryParams}`
    const decrees = await decrees_by_search(filter, fetch);
    const departmentsPerGp = await departments_per_gp(fetch);
    return { decrees, selectedGp: searchParams.get("decree[gp][in][0]"), departmentsPerGp }
}
