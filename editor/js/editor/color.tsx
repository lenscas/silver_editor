import * as React from "react"
import { EditableComponent } from "./basic_editor"
import { process_color_event } from "../app"



export class Color extends React.Component {
	render() {

		return <div>
			<input type="color" onChange={(e) => {
				process_color_event(e.target.value)
			}} />
		</div>
	}
}