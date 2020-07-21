import * as React from "react"
import { BasicForm, number_validation, BasicFormProps, always } from "../components/basic_form"
import { add_event_to_queue } from "../app"
import { v4 } from "uuid"
import { AddRectangleEvent, EditRectangleEvent } from "../events"
import { EditRectangle, IncommingEvents } from "../incoming_events/incoming_events"

type Rect = {
	color: string,
	length_x: number,
	length_y: number,
	pos_x: number,
	pos_y: number
}

export type RectangleProps = {
	editData?: EditRectangle["EditRectangle"]
	goToNextScreen: (_: IncommingEvents) => void
}

export class Rectangle extends React.Component<RectangleProps> {
	render() {
		//const fields: BasicFormProps<Rect>["inputs"] =
		return <div>
			<BasicForm<Rect> inputs={[
				{
					name: "color",
					type: "color",
					validation: always,
					start_value: this.props.editData?.color

				},
				{
					name: "length_x",
					type: "number",
					validation: number_validation,
					start_value: this.props.editData?.size[0]
				},
				{
					name: "length_y",
					type: "number",
					validation: number_validation,
					start_value: this.props.editData?.size[1]
				}, {
					name: "pos_x",
					type: "number",
					validation: number_validation,
					start_value: this.props.editData?.location[0]
				},
				{
					name: "pos_y",
					type: "number",
					validation: number_validation,
					start_value: this.props.editData?.location[1]
				}
			]
			} on_submit={x => {
				let id = this.props.editData?.id || v4()
				let event_type: AddRectangleEvent["event_type"] | EditRectangleEvent["event_type"] = this.props.editData?.id ? "EditRectangle" : "AddRectangle"
				let event: AddRectangleEvent | EditRectangleEvent = {
					event_type,
					params: {
						color: x.get("color") as string,
						size: [Number(x.get("length_x")), Number(x.get("length_y"))],
						location: [Number(x.get("pos_x")), Number(x.get("pos_y"))],
						id
					}
				}
				add_event_to_queue(event).then(() => {
					this.props.goToNextScreen({ EditRectangle: event.params })
				})
			}} />
		</div>
	}
}