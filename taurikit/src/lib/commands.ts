import type { Filter } from '../../timespent/bindings/Filter';
import type { ScaleXSegments } from '../../timespent/bindings/ScaleXSegments';
import type { YActivities } from '../../timespent/bindings/YActivities';

import { invoke } from '@tauri-apps/api/tauri';

export async function getGraph(): Promise<[number, ScaleXSegments, YActivities]> {
	const [scale_x_segments, y_activities]: [ScaleXSegments, YActivities] = await invoke(
		'get_graph',
		{}
	);

	const total_minutes_all = Number(y_activities.scale_total_minutes['All'][0]);

	return [total_minutes_all, scale_x_segments, y_activities];
}

export async function getFilter(): Promise<[ScaleXSegments, Filter, Filter]> {
	const res: [ScaleXSegments, Filter, Filter] = await invoke('get_filter', {});

	return [res[0], res[1], res[2]];
}

export async function applyFilter(ftr: Filter): Promise<void> {
	await invoke('apply_filter', {
		filter: ftr
	});

	return;
}
