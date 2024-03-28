pub use log::*;
use tokio::task;
use tower_lsp::{lsp_types::MessageType, Client};

struct LspLogger {
    client: Client,
}

impl LspLogger {
    pub fn new(client: Client) -> Self {
        LspLogger { client }
    }
}

impl log::Log for LspLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let typ = match record.level() {
            Level::Error => MessageType::ERROR,
            Level::Warn => MessageType::WARNING,
            Level::Info => MessageType::INFO,
            Level::Debug => MessageType::LOG,
            Level::Trace => MessageType::LOG,
        };
        let message = record.args().to_string();
        let client = self.client.clone();
        task::spawn(async move {
            client.log_message(typ, message).await;
        });
    }

    fn flush(&self) {}
}

pub fn init_logging(client: Client) {
    let logger = LspLogger::new(client);
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(Level::Trace.to_level_filter());
}
