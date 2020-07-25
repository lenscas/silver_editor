use super::shared_event_logic::json_value_iter_to_event_iter;
use silver_editor_event_types::{Event, SendEvents};

use crate::EditorConfig;
use bytes::Buf;
use serde_json::value::Value;
use std::{
    sync::{Arc, Mutex},
    thread::JoinHandle,
};
use tokio::{
    runtime::{self, Runtime},
    sync::{mpsc, RwLock},
};
use warp::{reply::html, ws::Message, Filter};

use futures::{FutureExt, StreamExt};

pub(crate) struct EventStream {
    _warp_thread: JoinHandle<()>,
    events: Arc<Mutex<Vec<Value>>>,
    editor_handles: EditorHandles,
}

type EditorHandles = Arc<RwLock<Vec<mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

impl EventStream {
    pub(crate) fn new(config: EditorConfig) -> Self {
        let events: Arc<Mutex<Vec<Value>>> = Default::default();
        let local_copy = events.clone();
        let editors: EditorHandles = Default::default();
        let editors_2 = editors.clone();
        let thread = std::thread::spawn(move || {
            let mut rt = Runtime::new().expect("Could not create tokio runtime :(");
            println!(
                "starting editor at `http://{:0}:{:1}`",
                config
                    .native_config
                    .address
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join("."),
                config.native_config.port
            );
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
                        .or(warp::path("ws")
                            .and(warp::ws())
                            .map(move |ws: warp::ws::Ws| {
                                let editors = editors.clone();
                                ws.on_upgrade(move |socket| async move {
                                    let (editor_ws_tx, _) = socket.split();
                                    let (tx, rx) = mpsc::unbounded_channel();
                                    tokio::task::spawn(rx.forward(editor_ws_tx).map(|result| {
                                        if let Err(e) = result {
                                            eprintln!("Websocket send error: {}", e)
                                        }
                                    }));
                                    let len = editors.read().await.len();
                                    editors.write().await.insert(len, tx);
                                    //maybe try looping over a channel here that tells it what to send instead of working with an array of channels
                                })
                            }))
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
                                                println!(
                                                    "Could not turn event body into string {:?}",
                                                    x
                                                );
                                                return "failed";
                                            }
                                            Ok(x) => x,
                                        };
                                        let x = match serde_json::from_str::<serde_json::Value>(
                                            &body,
                                        ) {
                                            Ok(x) => x,
                                            Err(x) => {
                                                println!(
                                                    "Could not parse event as json error : {:?}",
                                                    x
                                                );
                                                return "failed";
                                            }
                                        };
                                        match local_copy.try_lock() {
                                            Ok(mut y) => {
                                                y.push(x);
                                            }
                                            Err(x) => {
                                                println!("Could not push event. Error : {:?}", x);
                                                return "failed";
                                            }
                                        }
                                        "awesome"
                                    },
                                ),
                            ))),
                )
                .run((config.native_config.address, config.native_config.port)),
            );
        });
        EventStream {
            _warp_thread: thread,
            events,
            editor_handles: editors_2,
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
    pub(crate) fn send_event(&mut self, event: SendEvents) {
        let mut basic_rt = runtime::Builder::new()
            .basic_scheduler()
            .build()
            .expect("Couldn't create a simple tokio runtime.");
        basic_rt.block_on(async {
            let mut handlers = self.editor_handles.write().await;
            for handle in handlers.iter_mut() {
                //TODO handle this error in a good way
                let _ = handle.send(Ok(Message::text(
                    serde_json::to_string(&event).expect("Could not serialize event"),
                )));
                println!("got here?")
            }
        })
    }
}
