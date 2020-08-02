import * as React from "react";
import { Menu } from "./menu";
import { Color } from "./color";
import { Rectangle } from "./rectangle";
import {Image} from "./image";
import {
  Header,
  RenderName,
  PossibleTheme,
  default_theme,
} from "../components/header";

import { make_event_stream } from "../event_stream";
import { SendEvents, AddRectangle } from "../generated/incomming_events";
import { ImageParams } from "../generated/outgoing_events";

export type EditableComponent = "color" | "AddRectangle" | "nothing" | "Image";

type BasicEditorState = {
  current_window: EditableComponent;
  selected_theme: PossibleTheme;
  edit_params?: SendEvents;
};

export class BasicEditor extends React.Component<{}, BasicEditorState> {
  constructor(props: {}) {
    super(props);
    let v = localStorage.getItem("editor_last_selected");
    let selected = default_theme;
    if (v) {
      let x = JSON.parse(v) as { name: string; link: string };
      selected = {
        is_default: false,
        name: x.name,
        element: <link rel="stylesheet" href={x.link} />,
        link: x.link,
      };
    }
    this.state = {
      current_window: "nothing",
      selected_theme: selected,
    };
    make_event_stream(this.dealWithIncomingEvents);
  }
  dealWithIncomingEvents = (event: SendEvents) => {
    this.setState((state) => ({
      ...state,
      current_window: (()=>{
        if(event.EditImage) {
          return "Image"
        } else if(event.EditRectangle){
          return "AddRectangle"
        } else {
          return "nothing"
        }
      })(),
      edit_params: event,
    }));
  };

  RenderCorrectEditor = (selected: EditableComponent) => {
    switch (selected) {
      case "color":
        return <Color />;
      case "Image":
        if(this.state.edit_params && "EditImage" in this.state.edit_params){
          const params  = this.state.edit_params.EditImage as any as ImageParams;
          return <Image goToNextScreen={this.dealWithIncomingEvents} editData={params}/>
        }
        return <Image goToNextScreen={this.dealWithIncomingEvents} />
      case "AddRectangle":
        if (this.state.edit_params && "EditRectangle" in this.state.edit_params) {
          const params = this.state.edit_params.EditRectangle as any as AddRectangle
          return (
            <Rectangle
              key={params.id}
              goToNextScreen={this.dealWithIncomingEvents}
              editData={params}
            />
          );
        } else {
          return (
            <Rectangle
              goToNextScreen={this.dealWithIncomingEvents}
              key="No-id"
            />
          );
        }

      case "nothing":
        return <></>;
      default:
        console.error("invalid selection. Got :", selected);
        return <></>;
    }
  };
  set_theme(theme: PossibleTheme) {
    this.setState((state) => {
      if (theme.is_default) {
        localStorage.removeItem("editor_last_selected");
      } else {
        localStorage.setItem(
          "editor_last_selected",
          JSON.stringify({ name: theme.name, link: theme.link })
        );
      }
      return { ...state, selected_theme: theme };
    });
  }
  new_screen = (new_screen: EditableComponent) => {
    this.setState((state) => ({
      ...state,
      current_window: new_screen,
      edit_params: undefined,
    }));
  };
  render() {
    return (
      <div
        className="container-fluid"
        style={{ maxHeight: "100vh", paddingLeft: "0px", paddingRight: "0px" }}
      >
        {this.state.selected_theme.element}
        <Header
          current={this.state.selected_theme.name}
          set_theme={(t) => this.set_theme(t)}
        />
        <div
          className="row"
          style={{ height: "100vh", maxHeight: "100vh", marginRight: "0px" }}
        >
          <div className="menu col-2">
            <Menu
              selected={this.state.current_window}
              select_window={this.new_screen}
            />
          </div>
          <div
            className="editor col"
            style={{ maxHeight: "100vh", overflow: "auto" }}
          >
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
    );
  }
}
