export { render_editor } from "./editor"

type EditorSpace = {
	internal: {
		window?: Window | null
	}
}

type WindowWithEditor = Window & { silver_editor: EditorSpace }

type Event = {
	event_type: "color"
	params: String
}

declare let silver_editor: {
	internal: {
		window?: Window | null
		events: Array<Event>
	}
}

const get_silver_editor = () => {
	if (silver_editor && !silver_editor.internal) {

		silver_editor.internal = {
			events: [],
		}
	}
	return silver_editor
}

export const get_events = () => {
	const editor = get_silver_editor()
	const events = [];
	while (true) {
		const event = editor.internal.events.pop()
		if (event) {
			events.push(event)
		} else {
			return events
		}
	}

}

export const setup_extra_window_button = (contents: string) => {
	const button = document.createElement("button")
	button.append("Create editor")
	button.addEventListener("click", () => {
		const editor = get_silver_editor()
		if (editor.internal.window) {
			return;
		}
		//a quick hack to get arround the "no implicit any problem".
		//normally, a window has no "silver_editor" field, but we need to hack one in to share memory with the second window
		const window2 = window.open("", "editor") as unknown as WindowWithEditor
		if (window2) {
			if (!("silver_editor" in window2)) {
				window2["silver_editor"] = editor
			}

			window2.document.write(contents)

		}
	})
	document.getElementsByTagName("body")[0].append(button)
}
export const hello_from_second_page = () => console.log("nice?")


export const add_event_to_queue = (e: Event) => {
	const editor = get_silver_editor()
	editor.internal.events.push(e)
}

export const process_color_event = (e: string) => {
	add_event_to_queue({
		event_type: "color",
		params: e
	})
}