import * as React from "react"
import { BasicForm, number_validation, BasicFormProps, always } from "../components.tsx/basic_form"
import { add_event_to_queue } from "../app"

type Rect = {
	color: string,
	length_x: number,
	length_y: number,
	pos_x: number,
	pos_y: number
}


export class Rectangle extends React.Component {
	render() {
		//const fields: BasicFormProps<Rect>["inputs"] =
		return <div>
			<h2>Create a rectangle</h2>
			<BasicForm<Rect> inputs={[
				{
					name: "color",
					type: "color",
					validation: always
				},
				{
					name: "length_x",
					type: "number",
					validation: number_validation
				},
				{
					name: "length_y",
					type: "number",
					validation: number_validation
				}, {
					name: "pos_x",
					type: "number",
					validation: number_validation
				},
				{
					name: "pos_y",
					type: "number",
					validation: number_validation
				}
			]
			} on_submit={x => {
				add_event_to_queue({
					event_type: "AddRectangle",
					params: {
						color: x.get("color") as string,
						size: [Number(x.get("length_x")), Number(x.get("length_y"))],
						location: [Number(x.get("pos_x")), Number(x.get("pos_y"))]
					}
				})
			}} />
		</div>
	}
}