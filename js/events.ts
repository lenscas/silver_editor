export type ColorEvent = {
	event_type: "color"
	params: String
}
export type AddRectangleEvent = {
	event_type: "AddRectangle"
	params: {
		color: string,
		location: [number, number],
		size: [number, number],
		id: string
	}
}
export type EditRectangleEvent = {
	event_type: "EditRectangle"
	params: {
		color: string,
		location: [number, number],
		size: [number, number],
		id: string
	}
}


export type Event = ColorEvent | AddRectangleEvent | EditRectangleEvent