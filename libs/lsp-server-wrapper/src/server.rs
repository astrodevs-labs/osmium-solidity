use std::cell::RefCell;
use std::error::Error;

use std::rc::{Rc, Weak};

use crate::service::LspService;
use crate::{Client, LanguageServer};
use lsp_server::{Connection, IoThreads, Message, Response};

pub(crate) trait LspServer {
    fn send(&self, msg: Message);
}

struct InnerLspStdioServer {
    connection: Connection,
    io_threads: IoThreads,
    client: Rc<RefCell<Client>>,
    self_ref: Weak<LspStdioServer>,
}

pub struct LspStdioServer {
    inner: InnerLspStdioServer,
}

impl<'a> LspStdioServer {
    pub fn new() -> Rc<LspStdioServer> {
        let (connection, io_threads) = Connection::stdio();
        let client = Rc::new(RefCell::new(Client::new()));
        Rc::new_cyclic(|me| LspStdioServer {
            inner: InnerLspStdioServer {
                connection,
                io_threads,
                client,
                self_ref: me.clone(),
            },
        })
    }

    fn run_initialization<S: LanguageServer>(
        &self,
        service: &LspService<S>,
    ) -> Result<(), Box<dyn Error>> {
        let (initialize_id, initialize_params) = self.inner.connection.initialize_start()?;
        let res = service.call_request("initialize", initialize_params)?;
        let res = match res {
            Some(res) => res,
            None => return Ok(()),
        };

        let resp = Response::new_ok(initialize_id, res);
        self.inner.connection.sender.send(resp.into()).unwrap();
        match &self.inner.connection.receiver.recv() {
            Ok(Message::Notification(n)) => service
                .call_notification(&n.method, n.params.clone())
                .map_err(|e| {
                    eprintln!("Error: {}", e);
                    e
                }),
            Ok(msg) => {
                return Err(
                    format!(r#"expected initialized notification, got: {msg:?}"#)
                        .to_owned()
                        .into(),
                );
            }
            Err(e) => {
                return Err(
                    format!("expected initialized notification, got error: {e}",)
                        .to_owned()
                        .into(),
                )
            }
        }?;

        Ok(())
    }

    pub fn serve<S: LanguageServer, F>(this: Rc<Self>, init: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(Rc<RefCell<Client>>) -> S + 'a,
    {
        this.inner
            .client
            .borrow_mut()
            .set_server(this.inner.self_ref.clone());
        let client = this.inner.client.clone();
        let service = LspService::new(client, init);
        this.run_initialization(&service)?;
        this.serve_loop(service)?;
        Rc::into_inner(this).unwrap().inner.io_threads.join()?;
        Ok(())
    }

    fn serve_loop<S: LanguageServer>(&self, service: LspService<S>) -> Result<(), Box<dyn Error>> {
        for msg in &self.inner.connection.receiver {
            match msg {
                Message::Request(req) => {
                    let id = req.id.clone();
                    let is_shutdown = self.inner.connection.handle_shutdown(&req);
                    let is_shutdown = match is_shutdown {
                        Ok(is_shutdown) => is_shutdown,
                        Err(e) => {
                            eprintln!("Handle shutdown Error: {}", e);
                            continue;
                        }
                    };
                    let result = service.call_request(&req.method, req.params);
                    let result = match result {
                        Ok(result) => result,
                        Err(e) => {
                            eprintln!("Call request Error: {}", e);
                            continue;
                        }
                    };
                    let resp = Response {
                        id,
                        result,
                        error: None,
                    };
                    if is_shutdown {
                        eprintln!("Shutting down");
                        return Ok(());
                    }
                    let status = self.inner.connection.sender.send(Message::Response(resp));
                    if status.is_err() {
                        eprintln!("Error sending request's response: {status:?}");
                    }
                    continue;
                }
                Message::Response(resp) => service.call_response(resp.id, resp.result),
                Message::Notification(not) => {
                    let status = service.call_notification(&not.method, not.params);
                    if status.is_err() {
                        eprintln!("Error calling notification ({}): {status:?}", not.method);
                    }
                    continue;
                }
            }
        }
        eprintln!("Out of loop");
        Ok(())
    }
}

impl LspServer for LspStdioServer {
    fn send(&self, msg: Message) {
        let _ = self.inner.connection.sender.send(msg).map_err(|e| {
            eprintln!("Error: {}", e);
        });
    }
}
