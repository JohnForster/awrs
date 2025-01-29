// This will be necessary for a wasm build.

#[cfg(target_arch = "wasm32")]
mod wasm_websocket {
    use std::{cell::RefCell, collections::VecDeque, rc::Rc};

    use bevy::log::info;
    use web_sys::{
        js_sys::{ArrayBuffer, Uint8Array},
        wasm_bindgen::{prelude::Closure, JsCast},
        BinaryType, Event, MessageEvent,
    };

    pub struct Client {
        pub socket: web_sys::WebSocket,
        pub recv_queue: Rc<RefCell<VecDeque<Vec<u8>>>>,
        _open_cb: Closure<dyn FnMut(Event)>,
        _message_cb: Closure<dyn FnMut(MessageEvent)>,
    }

    impl Client {
        pub fn new(url: &str) -> send_wrapper::SendWrapper<Self> {
            info!("Opening wasm websocket");
            let recv_queue = Rc::new(RefCell::new(VecDeque::new()));
            let socket = web_sys::WebSocket::new(url).expect("Failed to create WebSocket object");
            socket.set_binary_type(BinaryType::Arraybuffer);
            let open_cb: Closure<dyn FnMut(_)> = Closure::new(|_event: Event| {
                web_sys::console::log_1(&"Connection opened".into());
            });
            socket
                .add_event_listener_with_callback("open", open_cb.as_ref().dyn_ref().unwrap())
                .unwrap();
            let message_cb: Closure<dyn FnMut(_)> = Closure::new({
                let recv_queue = Rc::clone(&recv_queue);
                move |event: MessageEvent| {
                    web_sys::console::log_1(&format!("Got message: {:?}", event.data()).into());
                    if let Some(buf) = event.data().dyn_ref::<ArrayBuffer>() {
                        recv_queue
                            .borrow_mut()
                            .push_back(Uint8Array::new(buf).to_vec());
                    }
                }
            });
            socket
                .add_event_listener_with_callback("message", message_cb.as_ref().dyn_ref().unwrap())
                .unwrap();
            send_wrapper::SendWrapper::new(Client {
                socket,
                recv_queue,
                _open_cb: open_cb,
                _message_cb: message_cb,
            })
        }
    }
}
