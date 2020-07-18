import * as React from "react"
import { Menu } from "./menu"
import { Color } from "./color"
import { Rectangle } from "./rectangle"
import { Header, RenderName, PossibleTheme, default_theme } from "../components/header"

import { make_event_stream } from "../incoming_events/websocket_client"
import { IncommingEvents } from "../incoming_events/incoming_events"

export type EditableComponent = "color" | "AddRectangle" | "nothing"

type BasicEditorState = {
	current_window: EditableComponent
	selected_theme: PossibleTheme
	edit_params?: IncommingEvents
}

export class BasicEditor extends React.Component<{}, BasicEditorState> {
	constructor(props: {}) {
		super(props)
		let v = localStorage.getItem("editor_last_selected")
		let selected = default_theme;
		if (v) {
			let x = JSON.parse(v) as { name: string, link: string }
			selected = {
				is_default: false,
				name: x.name,
				element: <link rel="stylesheet" href={x.link} />,
				link: x.link
			}
		}
		this.state = {
			current_window: "nothing", selected_theme: selected
		}
		make_event_stream((event) => {
			this.setState(state => ({
				...state,
				current_window: event.EditRectangle ? "AddRectangle" : "nothing",
				edit_params: event
			}))
		})
	}

	RenderCorrectEditor = (selected: EditableComponent) => {
		switch (selected) {
			case "color":
				return <Color />
			case "AddRectangle":
				if (this.state.edit_params && this.state.edit_params.EditRectangle) {
					return <Rectangle editData={this.state.edit_params.EditRectangle} />
				} else {
					return <Rectangle />
				}

			case "nothing":
				return <></>
			default:
				console.error("invalid selection. Got :", selected)
				return <></>
		}
	}
	set_theme(theme: PossibleTheme) {
		this.setState(state => {
			if (theme.is_default) {
				localStorage.removeItem("editor_last_selected")
			} else {
				localStorage.setItem("editor_last_selected", JSON.stringify({ name: theme.name, link: theme.link }))
			}
			return { ...state, selected_theme: theme }

		})
	}
	render() {
		return <div className="container-fluid" style={{ maxHeight: "100vh", paddingLeft: "0px", paddingRight: "0px" }}>
			{this.state.selected_theme.element}
			<Header current={this.state.selected_theme.name} set_theme={(t) => this.set_theme(t)} />
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
							{this.RenderCorrectEditor(this.state.current_window)}
						</div>
					</div>
				</div>
			</div>
		</div>
	}
}