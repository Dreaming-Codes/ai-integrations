// place files you want to import through the `$lib` alias in this folder.
export function cssVarToRGBA(color: string, opacity: number) {
	return `rgba(${getComputedStyle(document.body)
		.getPropertyValue(color)
		.split(' ')
		.join(',')}, ${opacity})`;
}
