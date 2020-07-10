import * as React from "react"
import { EditableComponent } from "./basic_editor"
import { process_color_event } from "../app"



export class Color extends React.Component {
	render() {

		return <div>
			<h2>Select background color</h2>
			<input type="color" onChange={(e) => {
				process_color_event(e.target.value)
			}} />
		</div>
	}
}