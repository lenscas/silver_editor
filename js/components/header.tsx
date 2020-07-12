import * as React from "react"
import { EditableComponent } from "../editor/basic_editor"

type ApiResponse = {
	name: string, cssCdn: string,
}

export type PossibleTheme = {
	name: string,
	element: JSX.Element
}

type HeaderState = {
	options?: PossibleTheme[]
}
type HeaderProps = { set_theme: (theme: PossibleTheme) => void, current: string }

export class Header extends React.Component<HeaderProps, HeaderState> {
	constructor(props: HeaderProps) {
		super(props)
		this.state = {}
		this.get_themes()
	}
	async get_themes() {
		const res = await fetch("https://bootswatch.com/api/4.json")
		const body = await res.json() as { themes: ApiResponse[] }
		this.setState(state => ({
			...state, options: body.themes.map(x => ({
				name: x.name,
				element: <link rel="stylesheet" href={x.cssCdn} crossOrigin="anonymous" />
			}
			))
		}))
	}
	render_options() {
		if (this.state.options) {
			return <ul className="navbar-nav">
				<li className="nav-item dropdown">
					<a className="nav-link dropdown-toggle" href="#" id="navbarDropdownMenuLink" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
						Themes</a>
					<div className="dropdown-menu" aria-labelledby="navbarDropdownMenuLink">
						{this.state.options.map(element => <div key={element.name} onClick={() => this.props.set_theme(element)}>{element.name}</div>)}
					</div>
				</li>
			</ul>
		}
	}
	render() {
		return <nav className="navbar navbar-expand-lg navbar-dark bg-dark">{this.render_options()}</nav>
	}
	/*
export const Header = ({set_theme, current}:) => {
return <nav className="navbar navbar-expand-lg navbar-dark bg-dark">

			<ul className="navbar-nav">
				<li className="nav-item dropdown">
					<a className="nav-link dropdown-toggle" href="#" id="navbarDropdownMenuLink" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
						Themes</a>

					<a className="dropdown-item" href="#">Default</a>
					<a className="dropdown-item" href="#">Dark</a>
					<a className="dropdown-item" href="#"></a>
				</div>
			</li>
			<script>console.log($("body"))</script>
		</ul>
	</div>
</nav >
}*/
}
export const RenderName = (props: { name: EditableComponent }) => {
	if (props.name == "nothing") {
		return <></>
	} else {
		return <span>{props.name}</span>
	}

}