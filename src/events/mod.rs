pub(crate) mod event_params;
mod send_events;
mod shared_event_logic;

#[cfg(not(target_arch = "wasm32"))]
mod get_events_native;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use get_events_native::EventStream;

#[cfg(target_arch = "wasm32")]
mod get_events_web;
#[cfg(target_arch = "wasm32")]
pub(crate) use get_events_web::EventStream;

pub(crate) use send_events::SendEvents;
pub(crate) use shared_event_logic::Event;
