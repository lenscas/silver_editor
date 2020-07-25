import * as React from "react";

export type ErrorOrSuccess = true | string;

export const always = (_?: string): ErrorOrSuccess => true;
export const number_validation = (x?: string) => {
  if (isNaN(Number(x))) {
    return "field is not a number";
  } else {
    return true;
  }
};

export type Input<T extends { [key: string]: unknown }> = {
  label?: string;
  name: keyof T & string;
  type: "text" | "color" | "number";
  validation: (_?: string) => ErrorOrSuccess;
  start_value?: string | number;
};
type BasicFormState<T extends { [key: string]: unknown }> = {
  values: Map<keyof T & string, string>;
};

export type BasicFormProps<T extends { [key: string]: unknown }> = {
  inputs: Array<Input<T>>;

  on_submit: (_: Map<keyof T & string, string>) => void;
};

const prettifyInputNames = (str: string) => {
  let x = str
    .split("_")
    .map((part) => {
      if (part.length == 1) {
        return part.toUpperCase();
      }
      return part;
    })
    .join(" ");
  x = x.charAt(0).toUpperCase() + x.slice(1);
  return x;
};
export class BasicForm<
  T extends { [key: string]: unknown }
> extends React.Component<BasicFormProps<T>, BasicFormState<T>> {
  constructor(props: BasicFormProps<T>) {
    super(props);
    const values = new Map();
    this.props.inputs.forEach((input) => {
      if (input.start_value) {
        values.set(input.name, input.start_value);
      }
    });
    this.state = { values };
  }
  render() {
    return (
      <form>
        {this.props.inputs.map((x) => {
          const update_state = (e: string) => {
            this.setState((old_state) => {
              let new_map = new Map();
              old_state.values.forEach((v, k) => new_map.set(k, v));
              new_map.set(x.name, e);
              return { ...old_state, values: new_map };
            });
          };
          const valid_classname =
            x.validation(this.state.values.get(x.name)) === true
              ? "is-valid"
              : "is-invalid";
          const render_error = () => {
            const validated = x.validation(this.state.values.get(x.name));
            if (validated === true) {
              return <></>;
            } else {
              return (
                <small className="form-text invalid-feedback">
                  {validated}
                </small>
              );
            }
          };
          const visibleInputName = x.label || prettifyInputNames(x.name);
          return (
            <div key={x.name} className="form-group row">
              <label className="col-md-2" htmlFor={x.name}>
                {visibleInputName}
              </label>
              <div className="col-md-10">
                <input
                  type={x.type}
                  className={"form-control " + valid_classname}
                  placeholder={visibleInputName}
                  value={this.state.values.get(x.name) || ""}
                  onChange={(e) => update_state(e.target.value)}
                />
                {render_error()}
              </div>
            </div>
          );
        })}
        <button
          className="btn btn-success"
          onClick={(e) => {
            e.preventDefault();
            if (
              this.props.inputs.every((x) =>
                x.validation(this.state.values.get(x.name))
              )
            ) {
              this.props.on_submit(this.state.values);
            }
          }}
        >
          Submit
        </button>
      </form>
    );
  }
}
