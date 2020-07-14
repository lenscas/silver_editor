# Silver_editor

Silver editor is a work in progress editor that allows you to sketch up an UI in your quicksilver program and play around with diffrent locations for shapes/images.
No more need to constantly change, compile, check and repeat every time you want to add something to the UI.

## External dependencies

 - [Yarn](https://yarnpkg.com/getting-started/install#global-install)
 - [Nodejs](https://nodejs.org/en/)
 
 These are needed because the GUI for the editor is written in typescript/react. Which is done so the editor can be enabled in WASM builds and doesn't need to make use of the same window as your game, preventing clutter.
 
 ## Build support

Silver editor works both if the game is compiled to wasm using `cargo web` and when it is compiled to native.
On the web, it injects a button into the page that when clicked opens a new page with the editor. (Element that gets the button injected is configurable)
When compiled to native, it opens up an HTTP server that serves the editor. (Port and binding ip are configurable)

An example on how to enable the editor is visible in [examples/test.rs](examples/test.rs). The example is also hosted at [https://lenscas.github.io/silver_editor/](https://lenscas.github.io/silver_editor/)

Support for `wasm-bindgen` is planned once quicksilver has fully documented that workflow.


