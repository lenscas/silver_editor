import { render_editor } from "./editor";
import "bootstrap";
import "jquery";

import { Event } from "./generated/outgoing_events";
import { SendEvents } from "./generated/incomming_events";

export { render_editor } from "./editor";

type EditorSpace = {
  internal: {
    window?: Window | null;
    events: Event[];
    has_shared_memory?: boolean;
    send_event?: (x: SendEvents) => void;
  };
};

type WindowWithEditor = Window & { silver_editor: EditorSpace };

declare global {
  interface Window {
    silver_editor: any;
  }
}

declare let silver_editor: {
  internal: {
    window?: Window | null;
    events: Array<Event>;
  };
};

export const get_silver_editor = (): EditorSpace => {
  if (silver_editor && !silver_editor.internal) {
    silver_editor.internal = {
      events: [],
    };
  }
  return silver_editor;
};

export const get_events = () => {
  const editor = get_silver_editor();
  const events = [];
  while (true) {
    const event = editor.internal.events.pop();
    if (event) {
      events.push(event);
    } else {
      return events;
    }
  }
};

export const setup_extra_window_button = (
  contents: string,
  element: HTMLElement
) => {
  if (!element) {
    throw new Error("Did not get an element.");
  }
  const editor = get_silver_editor();
  editor.internal.has_shared_memory = true;
  const button = document.createElement("button");
  button.append("Create editor");
  button.addEventListener("click", () => {
    //a quick hack to get arround the "no implicit any problem".
    //normally, a window has no "silver_editor" field, but we need to hack one in to share memory with the second window
    const window2 = (window.open("", "editor") as unknown) as WindowWithEditor;
    if (window2) {
      if (!("silver_editor" in window2)) {
        window2["silver_editor"] = editor;
        const jquery = Object.keys(window).find((x) => {
          console.log(x, x.startsWith("jQuery"));
          return x.startsWith("jQuery");
        });
        if (jquery) {
          (window2 as any)[jquery] = (window as any)[jquery];
        } else {
          console.error("Couldn't find jquery to move over :(");
        }
      }
      console.log(contents);
      //window2.document.write(basic_layout);
      window2.document.write(contents);
      render_editor(window2);
    }
  });
  element.append(button);
};
export const hello_from_second_page = () => console.log("nice?");

export const add_event_to_queue = async (e: Event) => {
  const editor = get_silver_editor();
  if (editor.internal.has_shared_memory) {
    editor.internal.events.push(e);
  } else {
    await fetch("event", { body: JSON.stringify(e), method: "POST" });
  }
};

export const process_color_event = (e: string) => {
  add_event_to_queue({
    Color: e,
  });
};
export const send_event = (e: SendEvents) => {
  const editor = get_silver_editor();
  if (editor.internal.send_event) {
    editor.internal.send_event(e);
  }
};
