import type { PageLoad } from './$types';

export const load: PageLoad = async ({ url }) => {
	return { status: url.searchParams.get('status') };
};
