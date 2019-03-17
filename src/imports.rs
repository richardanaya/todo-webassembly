extern "C" {
    pub fn global_getWindow() -> i32;
    pub fn global_createEventListener() -> i32;
    pub fn global_getProperty(obj: i32, name: i32) -> i32;
    pub fn EventTarget_addEventListener(element: i32, eventName: i32, callback: i32) -> i32;
    pub fn Element_set_innerHTML(element: i32, text: i32);
    pub fn Element_attachShadow(element: i32) -> i32;
    pub fn CustomElement_define(name: i32, attributes: i32);
}
