use crate::imports::*;
use webcomponent::*;

pub struct ToDo {
    element: i32,
}

impl ToDo {
    pub fn create(element: i32) {
        unsafe {
            get_components().push(ToDo { element: element });
            let id = get_components::<ToDo>().len() - 1;
            let cb = global_createEventListener();
            EventTarget_addEventListener(element, cstr("connected"), cb);
            add_callback(
                cb,
                Box::new(move |_| {
                    get_component::<ToDo>(id).connected();
                }),
            );
        }
    }

    fn connected(&self) {
        unsafe {
            let shadow = Element_attachShadow(self.element);
            Element_set_innerHTML(
                shadow,
                cstr("<style>:host{font-size:30px}</style><div>ToDo</div>"),
            );
        }
    }
}
