import * as React from "react";
import * as ReactDom from "react-dom";

export const render_editor = (window: Window) => {
  ReactDom.render(
    <h1>Hello from react</h1>,
    window.document.getElementById("react_handle")
  );
};
