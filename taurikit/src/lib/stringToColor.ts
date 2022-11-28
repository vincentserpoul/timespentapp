const stringToColor = function (str: string) {
	// let hash = 0;
	// for (let i = 0; i < str.length; i++) {
	// 	hash = str.charCodeAt(i) + ((hash << 5) - hash);
	// }

	// let colour = '#';
	// for (let i = 0; i < 3; i++) {
	// 	const value = (hash >> (i * 8)) & 0xff;
	// 	colour += ('ff' + value.toString(16)).substr(-2);
	// }

	// return colour;
	return stringToHslColor(str, 50, 60);
};

function stringToHslColor(str: string, s: number, l: number) {
	let total = 0;
	for (let i = 0; i < str.length; i++) {
		total += str.charCodeAt(i);
	}

	const h = total % 360;
	return 'hsl(' + h + ', ' + s + '%, ' + l + '%)';
}

export default stringToColor;
