import { IncommingEvents } from "./incoming_events"
export const make_event_stream = (deal_with_events: (x: IncommingEvents) => void) => {
	const url = "ws://" + location.hostname + (location.port ? ':' + location.port : '') + "/ws"
	const socket = new WebSocket(url)
	socket.addEventListener("message", x => {
		console.log(x)
		const data = JSON.parse(x.data) as IncommingEvents
		deal_with_events(data)

	})
}