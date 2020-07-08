use stdweb::{js, unstable::TryInto};
pub fn did_click_button() -> bool {
    let x = js! {
        if(window.did_press) {
            window.did_press = false;
            return true;
        }
        return false;
    };
    x.try_into().unwrap()
}
pub fn log(x: String) {
    js! {console.log(@{x})}
}
