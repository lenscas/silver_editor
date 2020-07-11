mod shared_event_logic;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod get_events_native;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use get_events_native as events;

#[cfg(target_arch = "wasm32")]
pub(crate) mod get_events_web;
#[cfg(target_arch = "wasm32")]
pub(crate) use get_events_web as events;
