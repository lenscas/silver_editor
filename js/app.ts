type EditorSpace = {
	internal: {
		window?: Window | null
	}
}
declare let silver_editor: {
	internal: {
		window?: Window | null
	}
}

const get_silver_editor = () => {
	if (silver_editor && !silver_editor.internal) {

		silver_editor.internal = {}
	}
	return silver_editor
}

export const setup_extra_window_button = (contents: string) => {

	const button = document.createElement("button")
	button.append("Create editor")
	button.addEventListener("click", () => {
		const editor = get_silver_editor()
		if (editor.internal.window) {
			return;
		}
		const window2 = window.open("", "editor")
		if (window2) {
			window2["silver_editor"] = editor
			window2.document.write(contents)

		}
	})
	document.getElementsByTagName("body")[0].append(button)
}
export const hello_from_second_page = () => console.log("nice?")