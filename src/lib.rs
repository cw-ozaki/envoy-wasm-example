use log::info;
use proxy_wasm::types::*;
use proxy_wasm::traits::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_stream_context(|context_id, _root_context_id| -> Box<dyn StreamContext> {
        Box::new(HelloWorld { context_id, bytes: Vec::new() } )
    })
}

struct HelloWorld {
    context_id: u32,
    bytes: Vec<u8>,
}

impl Context for HelloWorld {}

impl StreamContext for HelloWorld {
    fn on_downstream_data(&mut self, data_size: usize, end_of_stream: bool) -> Action {
        log::info!("context: {}, data_size: {}, end_of_stream, {}", self.context_id, data_size, end_of_stream);
        if let Some(mut v) = self.get_downstream_data(0, data_size) {
            self.bytes.append(&mut v)
        }
        if end_of_stream {
            log::info!("context: {}, {:?}", self.context_id, self.bytes);
        }

        Action::Continue
    }
}