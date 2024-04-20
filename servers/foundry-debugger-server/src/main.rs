use ::dap::requests::Command;
use ::dap::responses::ResponseBody;
use ::dap::server::Server;
use std::io::{BufReader, BufWriter, Read, Write};

mod dap;

type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn loop_uninitialized_dap<R: Read, W: Write>(mut server: Server<R, W>) -> DynResult<()> {
    loop {
        let req = match server.poll_request()? {
            Some(req) => req,
            None => break,
        };
        eprintln!("Received request: {:?}", req.command);

        match req.command {
            Command::Initialize(_) => {
                let rsp = req.success(ResponseBody::Initialize(Default::default()));
                server.respond(rsp)?;
            }

            Command::Launch(_) => {
                eprintln!("Received request: {:?}", req.command);
                server.respond(req.ack()?)?;
                dap::run_session(server)?;
                break;
            }

            Command::Disconnect(_) => {
                server.respond(req.ack()?)?;
                break;
            }

            _ => {
                let command = req.command;
                eprintln!("ERROR: unhandled command: {command:?}");
            }
        }
    }
    Ok(())
}

fn main() -> DynResult<()> {
    let output = BufWriter::new(std::io::stdout());
    let input = BufReader::new(std::io::stdin());
    let server = Server::new(input, output);

    // TODO handle launch command once we know what to send
    eprintln!("Waiting for initialize command");
    loop_uninitialized_dap(server)
}
