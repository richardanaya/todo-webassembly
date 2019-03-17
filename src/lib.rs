use imports::*;
use webcomponent::*;

mod components;
mod imports;

#[no_mangle]
pub fn main() -> () {
    unsafe {
        let win = global_getWindow();
        let cb = global_createEventListener();
        EventTarget_addEventListener(win, cstr("customelementcreated"), cb);
        add_callback(
            cb,
            Box::new(|event| {
                let element = global_getProperty(event, cstr("detail"));
                components::todo::XClock::create(element);
            }),
        );
        CustomElement_define(cstr("x-todo"), cstr("time"));
    }
}

#[no_mangle]
pub fn callback(callback_id: i32, event: i32) {
    // this function routes callbacks to the right closure
    route_callback(callback_id, event);
}
