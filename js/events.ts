import { AddRectangle } from "./generated/outgoing_events/add_rectangle"

export type ColorEvent = {
	event_type: "color"
	params: String
}
export type AddRectangleEvent = {
	event_type: "AddRectangle"
	params: AddRectangle
}
export type EditRectangleEvent = {
	event_type: "EditRectangle"
	params: AddRectangle
}


export type Event = ColorEvent | AddRectangleEvent | EditRectangleEvent