import * as React from "react";
import { BasicForm } from "../components/basic_form";
import { add_event_to_queue } from "../app";
import { v4 } from "uuid";
import {
  Event,
  Rectangle as Rec,
  AddRectangle,
} from "../generated/outgoing_events";
import { SendEvents } from "../generated/incomming_events";

type Rect = {
  color: string;
  length_x: number;
  length_y: number;
  pos_x: number;
  pos_y: number;
};

export type RectangleProps = {
  editData?: AddRectangle;
  goToNextScreen: (_: SendEvents) => void;
};

export class Rectangle extends React.Component<RectangleProps> {
  render() {
    //const fields: BasicFormProps<Rect>["inputs"] =
    return (
      <div>
        <BasicForm<Rect>
          inputs={[
            {
              name: "color",
              type: "color",
              start_value: this.props.editData?.color,
            },
            {
              name: "length_x",
              type: "number",
              start_value: this.props.editData?.rectangle.size.x,
              validation: {
                min: 0,
              },
            },
            {
              name: "length_y",
              type: "number",
              start_value: this.props.editData?.rectangle.size.y,
              validation: {
                min: 0,
              },
            },
            {
              name: "pos_x",
              type: "number",
              start_value: this.props.editData?.rectangle.pos.x,
            },
            {
              name: "pos_y",
              type: "number",
              start_value: this.props.editData?.rectangle.pos.y,
            },
          ]}
          trans={(k, v) => {
            switch (k) {
              case "color":
                return v.toString();
              case "length_x":
                return Number(v);
              case "length_y":
                return Number(v);
              case "pos_x":
                return Number(v);
              case "pos_y":
                return Number(v);
              default:
                return v as any;
            }
          }}
          on_submit={(x) => {
            const id = this.props.editData?.id || v4();
            const rectangle: Rec = {
              pos: { x: x.pos_x, y: x.pos_y },
              size: {
                x: x.length_x,
                y: x.length_y,
              },
            };
            const color = x.color;
            const params: AddRectangle = {
              color,
              id,
              rectangle,
            };

            if (!this.props.editData?.id) {
              var event: Event = {
                AddRectangle: params,
              };
            } else {
              var event: Event = {
                EditRectangle: params,
              };
            }
            add_event_to_queue(event).then(() => {
              this.props.goToNextScreen({ EditRectangle: params });
            });
          }}
        />
      </div>
    );
  }
}
