import type { XSegment } from '../../timespent/bindings/XSegment';

export function displayedXLabels(scaled_x_labels: XSegment[]): string[] {
	return scaled_x_labels.map((x_segment) => x_segment.start_datetime.split('T')[0]);
}
