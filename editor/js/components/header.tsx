import * as React from "react";
import { EditableComponent } from "../editor/basic_editor";

export const default_theme: PossibleTheme = {
  is_default: true,
  name: "default",
  element: (
    <link
      rel="stylesheet"
      href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.0/css/bootstrap.min.css"
      integrity="sha384-9aIt2nRpC12Uk9gS9baDl411NQApFmC26EwAOH8WgZl5MYYxFfc+NcPb1dKGj7Sk"
      crossOrigin="anonymous"
    />
  ),
};

type ApiResponse = {
  name: string;
  cssCdn: string;
};

export type PossibleTheme =
  | {
      is_default: false;
      name: string;
      link: string;
      element: JSX.Element;
    }
  | {
      is_default: true;
      name: "default";
      element: JSX.Element;
    };

type HeaderState = {
  options?: PossibleTheme[];
};
type HeaderProps = {
  set_theme: (theme: PossibleTheme) => void;
  current: string;
};

export class Header extends React.Component<HeaderProps, HeaderState> {
  constructor(props: HeaderProps) {
    super(props);
    this.state = {};
    this.get_themes();
  }
  async get_themes() {
    const res = await fetch("https://bootswatch.com/api/4.json");
    const body = (await res.json()) as { themes: ApiResponse[] };
    this.setState((state) => ({
      ...state,
      options: [default_theme].concat(
        body.themes.map((x) => ({
          is_default: false,
          link: x.cssCdn,
          name: x.name,
          element: (
            <link rel="stylesheet" href={x.cssCdn} crossOrigin="anonymous" />
          ),
        }))
      ),
    }));
  }
  render_options() {
    if (this.state.options) {
      return (
        <ul className="navbar-nav">
          <li className="nav-item dropdown">
            <a
              className="nav-link dropdown-toggle"
              href="#"
              id="navbarDropdownMenuLink"
              data-toggle="dropdown"
              aria-haspopup="true"
              aria-expanded="false"
            >
              Themes
            </a>
            <div
              className="dropdown-menu"
              aria-labelledby="navbarDropdownMenuLink"
            >
              {this.state.options.map((element) => (
                <div
                  key={element.name}
                  onClick={() => this.props.set_theme(element)}
                >
                  {element.name}
                </div>
              ))}
            </div>
          </li>
        </ul>
      );
    }
  }
  render() {
    return (
      <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
        {this.render_options()}
      </nav>
    );
  }
}

export const SplitStringAtCamelCalse = (str: string) => {
  const str_split = str.split(/(?=[A-Z])/);
  let new_str = str_split.shift();
  new_str += " " + str_split.map((v) => v.toLowerCase()).join(" ");
  return new_str;
};

export const RenderName = (props: { name: EditableComponent }) => {
  if (props.name == "nothing") {
    return <></>;
  } else {
    const name = SplitStringAtCamelCalse(props.name);
    return <span>{name}</span>;
  }
};
