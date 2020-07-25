import { SendEvents } from "./generated/incomming_events";
import { get_silver_editor } from "./app";

export const make_event_stream = (
  deal_with_events: (x: SendEvents) => void
) => {
  const editor = get_silver_editor();
  if (editor.internal.has_shared_memory) {
    make_event_stream_wasm(deal_with_events);
  } else {
    make_event_stream_native(deal_with_events);
  }
};
const make_event_stream_wasm = (deal_with_events: (x: SendEvents) => void) => {
  const editor = get_silver_editor();
  editor.internal.send_event = deal_with_events;
};

const make_event_stream_native = (
  deal_with_events: (x: SendEvents) => void
) => {
  const url =
    "ws://" +
    location.hostname +
    (location.port ? ":" + location.port : "") +
    "/ws";
  const socket = new WebSocket(url);
  socket.addEventListener("message", (x) => {
    console.log(x);
    const data = JSON.parse(x.data) as SendEvents;
    deal_with_events(data);
  });
};
