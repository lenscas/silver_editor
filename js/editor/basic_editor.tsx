import * as React from "react"
import { Menu } from "./menu"
import { Color } from "./color"

export type EditableComponent = "color" | "test"

type BasicEditorState = {
	current_window?: EditableComponent
}

const RenderCorrectEditor = (selected?: EditableComponent) => {
	switch (selected) {
		case "color":
			return <Color />
		default:
			return <></>
	}
}

export class BasicEditor extends React.Component<{}, BasicEditorState> {
	constructor(props: {}) {
		super(props)
		this.state = {}
	}
	render() {
		return <div className="container">
			<link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.0/css/bootstrap.min.css"
				integrity="sha384-9aIt2nRpC12Uk9gS9baDl411NQApFmC26EwAOH8WgZl5MYYxFfc+NcPb1dKGj7Sk" crossOrigin="anonymous" />
			<script src="https://code.jquery.com/jquery-3.5.1.slim.min.js"
				integrity="sha384-DfXdz2htPH0lsSSs5nCTpuj/zy4C+OGpamoFVy38MVBnE+IbbVYUew+OrCXaRkfj"
				crossOrigin="anonymous"></script>
			<script src="https://cdn.jsdelivr.net/npm/popper.js@1.16.0/dist/umd/popper.min.js"
				integrity="sha384-Q6E9RHvbIyZFJoft+2mJbHaEWldlvI9IOYy5n3zV9zzTtmI3UksdQRVvoxMfooAo"
				crossOrigin="anonymous"></script>
			<script src="https://stackpath.bootstrapcdn.com/bootstrap/4.5.0/js/bootstrap.min.js"
				integrity="sha384-OgVRvuATP1z7JjHLkuOU7Xw704+h835Lr+6QL9UvYjZE3Ipu6Tp75j7Bh/kR0JKI"
				crossOrigin="anonymous"></script>
			<div className="row">
				<div className="menu col-2">
					<Menu select_window={(a) => this.setState(x => ({ ...x, current_window: a }))} />
				</div>
				<div className="editor col">
					{RenderCorrectEditor(this.state.current_window)}
				</div>
			</div>
		</div>
	}
}