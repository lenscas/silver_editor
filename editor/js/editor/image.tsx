import * as React from "react";
import { BasicForm } from "../components/basic_form";
import { add_event_to_queue } from "../app";
import { v4 } from "uuid";
import {
  Event,
  Rectangle as Rec,
  AddRectangle,
  ImageParams,
} from "../generated/outgoing_events";
import { SendEvents } from "../generated/incomming_events";

type Img = {
  length_x: number;
  length_y: number;
  pos_x: number;
  pos_y: number;
  path: string;
};

export type ImageProps = {
  editData?: ImageParams;
  goToNextScreen: (_: SendEvents) => void;
};

export class Image extends React.Component<ImageProps> {
  render() {
    console.log(this.props)
    return (
      <div>
        <BasicForm<Img>
          inputs={[
            {
              name: "path",
              type: "text",
              start_value: this.props.editData?.image_path,
              validation: {
                custom: async (v)=>{
                  if(v.startsWith("/")){
                    v = v.substr(1)
                  }
                  const res = await fetch(v)
                  if(res.ok){
                    return {state:"valid"}
                  } else {
                    return {state:"invalid",message:"Could not find image at given path"}
                  }
                }
              }
            },
            {
              name: "length_x",
              type: "number",
              start_value: this.props.editData?.location.size.x,
              validation: {
                min: 0,
              },
            },
            {
              name: "length_y",
              type: "number",
              start_value: this.props.editData?.location.size.y,
              validation: {
                min: 0,
              },
            },
            {
              name: "pos_x",
              type: "number",
              start_value: this.props.editData?.location.pos.x,
            },
            {
              name: "pos_y",
              type: "number",
              start_value: this.props.editData?.location.pos.y,
            },
          ]}
          trans={(k, v) => {
            switch (k) {
              case "path":
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
            const location: Rec = {
              pos: { x: x.pos_x, y: x.pos_y },
              size: {
                x: x.length_x,
                y: x.length_y,
              },
            };
            const image_path = x.path;
            const params: ImageParams = {
              image_path,
              id,
              location,
            };

            if (!this.props.editData?.id) {
              var event: Event = {
                AddImage: params,
              };
            } else {
              var event: Event = {
                EditImage: params,
              };
            }
            add_event_to_queue(event).then(() => {

              this.props.goToNextScreen({ EditImage: params });
            });
          }}
        />
      </div>
    );
  }
}
