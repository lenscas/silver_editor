# silver_editor
A small editor for quicksilver and Mergui

Silver editor is a work in progress editor that allows you to sketch up an UI in your quicksilver program and play around with diffrent locations for shapes/images.
No more need to constantly change, compile, check and repeat every time you want to add something to the UI.

Silver editor works both if the game is compiled to wasm using `cargo web` and when it is compiled to native.
On the web, it injects a button into the page that when clicked opens a new page with the editor. 
When compiled to native, it opens up an HTTP server listening on port 3030 that serves the editor.

An example on how to enable the editor is visible in [examples/test.rs](examples/test.rs). The example is also hosted at [https://lenscas.github.io/silver_editor/](https://lenscas.github.io/silver_editor/)

Support for `wasm-bindgen` is planned once quicksilver has fully documented that workflow.
