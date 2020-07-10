import * as React from "react";
import * as ReactDom from "react-dom";
import { BasicEditor } from "./editor/basic_editor";

export const render_editor = (window: Window) => {
  ReactDom.render(
    <BasicEditor />,
    window.document.getElementById("react_handle")
  );
};
