use crate::imports::*;
use webcomponent::*;

pub struct XClock {
    element: i32,
}

impl XClock {
    pub fn create(element: i32) {
        unsafe {
            // store xclock and keep its index
            get_components().push(XClock { element: element });
            let id = get_components::<XClock>().len() - 1;

            let mut cb = global_createEventListener();
            EventTarget_addEventListener(element, cstr("connected"), cb);
            add_callback(
                cb,
                Box::new(move |_| {
                    get_component::<XClock>(id).connected();
                }),
            );

            cb = global_createEventListener();
            EventTarget_addEventListener(element, cstr("attributechanged"), cb);
            add_callback(
                cb,
                Box::new(move |event| {
                    get_component::<XClock>(id).attribute_changed(event);
                }),
            );
        }
    }

    fn connected(&self) {
        unsafe {
            let shadow = Element_attachShadow(self.element);
            Element_set_innerHTML(
                shadow,
                cstr("<style>:host{font-size:30px}</style><div>12:00 AM</div>"),
            );
        }
    }

    fn attribute_changed(&self, _event: i32) {
        unsafe { console_log(cstr("my attributes changed")) }
    }
}
