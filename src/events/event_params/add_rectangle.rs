use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct AddRectangle {
    pub color: String,
    pub location: (f32, f32),
    pub size: (f32, f32),
}
