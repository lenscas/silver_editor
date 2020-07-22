use quicksilver::geom::Vector;
use serde::{Deserialize, Serialize};
/*
#[derive(Deserialize, Serialize)]
pub(crate) struct AddRectangle {
    pub color: String,
    pub location: (f32, f32),
    pub size: (f32, f32),
    pub id: String,
}
*/
schemafy::schemafy!(
    root: AddRectangle
    "json_schemas/add_rectangle.json"
);

impl Into<Vector> for AddRectangleLocation {
    fn into(self) -> Vector {
        Vector::new(self.x as f32, self.y as f32)
    }
}
impl Into<Vector> for AddRectangleLocationSize {
    fn into(self) -> Vector {
        Vector::new(self.x as f32, self.y as f32)
    }
}
impl From<Vector> for AddRectangleLocationSize {
    fn from(a: Vector) -> Self {
        Self {
            x: a.x as f64,
            y: a.y as f64,
        }
    }
}

impl From<Vector> for AddRectangleLocation {
    fn from(a: Vector) -> Self {
        Self {
            x: a.x as f64,
            y: a.y as f64,
        }
    }
}
