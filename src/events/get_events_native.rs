use super::shared_event_logic::{json_value_iter_to_event_iter, Event};
use bytes::Buf;
use serde_json::value::Value;
use std::{
    sync::{Arc, Mutex},
    thread::JoinHandle,
};
use tokio::runtime::Runtime;
use warp::{reply::html, Filter};

pub(crate) struct EventStream {
    _warp_thread: JoinHandle<()>,
    events: Arc<Mutex<Vec<Value>>>,
}

impl EventStream {
    pub(crate) fn new() -> Self {
        let events: Arc<Mutex<Vec<Value>>> = Default::default();
        let local_copy = events.clone();
        let thread =
            std::thread::spawn(move || {
                let mut rt = Runtime::new().expect("Could not create tokio runtime :(");
                rt.block_on(
                warp::serve(
                    warp::any()
                        .and(warp::path("editor.js").and(warp::get()).map(|| {
                            let output = include_str!("../../build/app.js");
                            output
                        }))
                        .or(warp::path("app.js.map")
                            .and(warp::get())
                            .map(|| include_str!("../../build/app.js.map")))
                        .or(warp::path::end()
                            .and(warp::get())
                            .map(|| {
                                let output = include_str!("../../static/editor_native.html");
                                html(output)
                            })
                            .or(warp::post().and(
                                warp::path("event").and(warp::filters::body::bytes()).map(
									move |x: bytes::Bytes| {
									let x = String::from_utf8(x.bytes().to_vec());
									let body = match x {
										Err(x) => {
											println!("Could not turn event body into string {:?}",x );
											return "failed";
										},
										Ok(x) => x
									};
									let x = match serde_json::from_str::<serde_json::Value>(&body) {
										Ok(x) => x,
										Err(x) => {
											println!("Could not parse event as json error : {:?}",x);
											return "failed"
										}
									};
									match local_copy.try_lock() {
										Ok(mut y) => {
											y.push(x);
										},
										Err(x) => {println!("Could not push event. Error : {:?}",x);return "failed"}
									}
									"awesome"
								}
                                ),
                            ))),
                )
                .run(([0, 0, 0, 0], 3030)),
            );
            });
        EventStream {
            _warp_thread: thread,
            events,
        }
    }
    pub(crate) fn get_events(&mut self) -> impl Iterator<Item = Event> {
        let mut res = self
            .events
            .try_lock()
            .expect("Could not get access to the events");
        let len = res.len();
        let res: Vec<_> = res.drain(0..len).collect();
        json_value_iter_to_event_iter(res.into_iter())
    }
}
