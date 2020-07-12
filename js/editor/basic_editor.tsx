import * as React from "react"
import { Menu } from "./menu"
import { Color } from "./color"
import { Rectangle } from "./rectangle"
import { Header, RenderName, PossibleTheme } from "../components/header"

export type EditableComponent = "color" | "AddRectangle" | "nothing"

type BasicEditorState = {
	current_window: EditableComponent
	selected_theme: PossibleTheme
}

const RenderCorrectEditor = (selected?: EditableComponent) => {
	switch (selected) {
		case "color":
			return <Color />
		case "AddRectangle":
			return <Rectangle />
		case "nothing":
			return <></>
		default:
			console.error("invalid selection. Got :", selected)
			return <></>
	}
}

export class BasicEditor extends React.Component<{}, BasicEditorState> {
	constructor(props: {}) {
		super(props)
		this.state = {
			current_window: "nothing", selected_theme: {
				name: "default",
				element: < link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.0/css/bootstrap.min.css"
					integrity="sha384-9aIt2nRpC12Uk9gS9baDl411NQApFmC26EwAOH8WgZl5MYYxFfc+NcPb1dKGj7Sk" crossOrigin="anonymous" />
			}
		}
	}
	render() {
		return <div className="container-fluid" style={{ maxHeight: "100vh", paddingLeft: "0px", paddingRight: "0px" }}>
			{this.state.selected_theme.element}
			<Header current={this.state.selected_theme.name} set_theme={(theme) => this.setState(state => ({ ...state, selected_theme: theme }))} />
			<div className="row" style={{ height: "100vh", maxHeight: "100vh", marginRight: "0px" }}>
				<div className="menu col-2">
					<Menu selected={this.state.current_window} select_window={(a) => this.setState(x => ({ ...x, current_window: a }))} />
				</div>
				<div className="editor col" style={{ maxHeight: "100vh", overflow: "auto" }}>
					<div className="card" style={{ marginTop: "15px" }}>
						<div className="card-header">
							<RenderName name={this.state.current_window} />
						</div>
						<div className="card-body">
							{RenderCorrectEditor(this.state.current_window)}
						</div>
					</div>
				</div>
			</div>
		</div>
	}
}