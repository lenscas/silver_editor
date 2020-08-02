use quicksilver::{
    geom::{Rectangle, Vector},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(remote = "Vector", deny_unknown_fields)]
pub(crate) struct VectorDef {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(remote = "Rectangle", deny_unknown_fields)]
pub(crate) struct RectangleDef {
    #[serde(with = "VectorDef")]
    pos: Vector,
    #[serde(with = "VectorDef")]
    size: Vector,
}