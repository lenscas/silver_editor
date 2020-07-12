import * as React from "react"
import { EditableComponent } from "./basic_editor"

const Choice = (props: MenuProperties & { current: EditableComponent }) => {
	const click = () => props.select_window(props.current)
	const active_class = (() => {
		if (props.current == props.selected) {
			return "btn-secondary"
		} else {
			return "btn-dark"
		}
	})()
	return <li className=""><button className={"btn btn-block " + active_class} type="button" onClick={click}>{props.current}</button></li >
}

export type MenuProperties = {
	select_window: (name: EditableComponent) => void
	selected: EditableComponent
}


export class Menu extends React.Component<MenuProperties> {
	render() {
		return <ul className="nav nav-pills flex-column bg-dark" style={{ "height": "100vh" }} >
			<Choice
				{...this.props}
				current="color"
			/><Choice
				{...this.props}
				current="AddRectangle"
			/>
		</ ul>
	}
}