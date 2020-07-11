import * as React from "react"
import { EditableComponent } from "./basic_editor"

const Choice = (name: EditableComponent, chosen: (comp: EditableComponent) => void) => {
	const click = () => chosen(name)
	return <p><a href="#" onClick={click}>{name}</a></p>
}

export type MenuProperties = {
	select_window: (name: EditableComponent) => void
}


export class Menu extends React.Component<MenuProperties> {
	render() {
		return <div>
			{Choice("color", this.props.select_window)}
			{Choice("AddRectangle", this.props.select_window)}
		</div>
	}
}