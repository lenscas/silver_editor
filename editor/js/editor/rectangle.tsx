import * as React from "react";
import { BasicForm, number_validation, always } from "../components/basic_form";
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
              validation: always,
              start_value: this.props.editData?.color,
            },
            {
              name: "length_x",
              type: "number",
              validation: number_validation,
              start_value: this.props.editData?.rectangle.size.x,
            },
            {
              name: "length_y",
              type: "number",
              validation: number_validation,
              start_value: this.props.editData?.rectangle.size.y,
            },
            {
              name: "pos_x",
              type: "number",
              validation: number_validation,
              start_value: this.props.editData?.rectangle.pos.x,
            },
            {
              name: "pos_y",
              type: "number",
              validation: number_validation,
              start_value: this.props.editData?.rectangle.pos.y,
            },
          ]}
          on_submit={(x) => {
            const id = this.props.editData?.id || v4();
            const rectangle: Rec = {
              pos: { x: Number(x.get("pos_x")), y: Number(x.get("pos_y")) },
              size: {
                x: Number(x.get("length_x")),
                y: Number(x.get("length_y")),
              },
            };
            const color = x.get("color") as string;
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
