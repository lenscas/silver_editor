import * as React from "react";

export type ErrorOrSuccess = true | string;

export type form_type = (
  | {
      type: "number";
      validation?: {
        min?: number;
      };
    }
  | {
      type: "color";
      validation?: {};
    }
  | {
      type: "text";
      validation?: {
        pattern?: string;
      };
    }
) & {
  validation?: {
    required?: boolean;
    custom?: (
      value: string
    ) => Promise<{ state: "valid" } | { state: "invalid"; message: string }>;
  };
};

export type Input<T extends { [key: string]: unknown }> = {
  label?: string;
  name: keyof T & string;
  start_value?: string | number;
} & form_type;

type BasicFormState<T extends { [key: string]: unknown }> = {
  values: Map<
    keyof T,
    | {
        state: "running";
        cancel: () => void;
      }
    | {
        state: "valid";
      }
    | { state: "invalid"; message: string }
  >;
};

export type BasicFormProps<T extends { [key: string]: unknown }> = {
  inputs: Array<Input<T>>;

  on_submit: (_: Map<keyof T, string>) => void;
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
      <form
        onSubmit={(e) => {
          e.preventDefault();
          const as_map = new Map<keyof T, string>();
          const data = new FormData(e.target as HTMLFormElement);
          
          data.forEach((v, k) => {
            as_map.set(k, v as string);
          });
          this.props.on_submit(as_map);
        }}
      >
        {this.props.inputs.map((x) => {
          const update_state = (e: string) => {
            this.setState((old_state) => {
              let new_map = new Map();
              old_state.values.forEach((v, k) => new_map.set(k, v));
              new_map.set(x.name, e);
              return { ...old_state, values: new_map };
            });
          };
          const visibleInputName = x.label || prettifyInputNames(x.name);
          const validation : form_type["validation"] = { ...x.validation };
          validation.required =
            validation.required || validation.required == undefined;
          const custom_validation_given  = validation.custom || (async ()=> ({state : "valid"}));
          const custom_validation = (
            e: React.ChangeEvent<HTMLInputElement>
          ) => {
            e.persist()
            const name = e.target.name
            const state = this.state.values.get(name);
            const current_value = e.target.value;
            const target = e.target;

            if (state?.state == "running") {
              state.cancel();
            }
            let shouldCancel = false;
            this.setState(state => {
              state.values.set(e.target.name, {
                state: "running",
                cancel: () => {
                  shouldCancel = true;
                },
              });
              return state
            },async ()=>{
              const res = await custom_validation_given(current_value)
              if(!shouldCancel){
                if(res.state == "invalid"){
                  target.setCustomValidity(res.message)
                } else {
                  target.setCustomValidity("")
                }
                this.setState(state => {
                  state.values.set(name,res)
                  return state
                })
              }
            })

          };
          validation.custom = undefined;
          return (
            <div key={x.name} className="form-group row">
              <label className="col-md-2" htmlFor={x.name}>
                {visibleInputName}
              </label>
              <div className="col-md-10">
                <input
                  type={x.type}
                  name={x.name}
                  id={x.name}
                  className={"form-control"}
                  placeholder={visibleInputName}
                  onChange={(e) => update_state(e.target.value)}
                  defaultValue = {x.start_value}
                  {...validation}
                  onInput = {custom_validation}
                />
              </div>
            </div>
          );
        })}
        <button className="btn btn-success">Submit</button>
      </form>
    );
  }
}
