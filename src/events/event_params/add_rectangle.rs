use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct AddRectangle {
    pub color: String,
    pub location: (f32, f32),
    pub size: (f32, f32),
    pub id: String,
}
