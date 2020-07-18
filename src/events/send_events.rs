use super::event_params::AddRectangle;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) enum SendEvents {
    EditRectangle(AddRectangle),
}
