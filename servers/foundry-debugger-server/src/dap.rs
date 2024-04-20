use std::io::{Read, Write};

use dap::errors::ServerError;
use dap::prelude::Event;
use dap::requests::{Command, Request};
use dap::responses::{
    ContinueResponse, DisassembleResponse, ResponseBody, ScopesResponse, SetBreakpointsResponse,
    SetExceptionBreakpointsResponse, SetInstructionBreakpointsResponse, StackTraceResponse,
    ThreadsResponse,
};
use dap::server::Server;
use dap::types::{
    SteppingGranularity, Thread,
};

pub struct DapSession<R: Read, W: Write> {
    server: Server<R, W>,
    running: bool,
    next_breakpoint_id: i64,
}

impl<'a, R: Read, W: Write> DapSession<R, W> {
    pub fn new(
        server: Server<R, W>,
    ) -> Self {
        Self {
            server,
            running: false,
            next_breakpoint_id: 1,
        }
    }

    pub fn run_loop(&mut self) -> Result<(), ServerError> {
        self.running = true;

        self.server.send_event(Event::Initialized)?;

        while self.running {
            let req = match self.server.poll_request()? {
                Some(req) => req,
                None => break,
            };
            match req.command {
                Command::Disconnect(_) => {
                    eprintln!("INFO: ending debugging session");
                    self.server.respond(req.ack()?)?;
                    break;
                }
                Command::SetBreakpoints(_) => {
                    self.handle_set_source_breakpoints(req)?;
                }
                Command::SetExceptionBreakpoints(_) => {
                    self.handle_set_exceptions_breakpoints(req)?;
                }
                Command::SetInstructionBreakpoints(_) => {
                    self.handle_set_instruction_breakpoints(req)?;
                }
                Command::Threads => {
                    self.server.respond(req.success(ResponseBody::Threads(ThreadsResponse {
                        threads: vec![Thread { id: 0, name: "main".to_string() }],
                    })))?;
                }
                Command::StackTrace(_) => {
                    self.handle_stack_trace(req)?;
                }
                Command::Disassemble(_) => {
                    self.handle_disassemble(req)?;
                }
                Command::StepIn(ref args) => {
                    let granularity =
                        args.granularity.as_ref().unwrap_or(&SteppingGranularity::Statement);
                    match granularity {
                        SteppingGranularity::Instruction => self.handle_step(req)?,
                        _ => self.handle_next(req)?,
                    }
                }
                Command::StepOut(ref args) => {
                    let granularity =
                        args.granularity.as_ref().unwrap_or(&SteppingGranularity::Statement);
                    match granularity {
                        SteppingGranularity::Instruction => self.handle_step(req)?,
                        _ => self.handle_next(req)?,
                    }
                }
                Command::Next(ref args) => {
                    let granularity =
                        args.granularity.as_ref().unwrap_or(&SteppingGranularity::Statement);
                    match granularity {
                        SteppingGranularity::Instruction => self.handle_step(req)?,
                        _ => self.handle_next(req)?,
                    }
                }
                Command::Continue(_) => {
                    self.handle_continue(req)?;
                }
                Command::Scopes(_) => {
                    let scopes = vec![];

                    self.server.respond(
                        req.success(ResponseBody::Scopes(ScopesResponse { scopes })),
                    )?;
                }
                _ => {
                    eprintln!("ERROR: unhandled command: {:?}", req.command);
                }
            }
        }
        Ok(())
    }

    fn handle_stack_trace(&mut self, req: Request) -> Result<(), ServerError> {
        let stack_frames = vec![];
        let total_frames = Some(stack_frames.len() as i64);
        self.server.respond(req.success(ResponseBody::StackTrace(StackTraceResponse {
            stack_frames,
            total_frames,
        })))?;
        Ok(())
    }

    fn handle_scopes(&mut self, req: Request) -> Result<(), ServerError> {
        let scopes = vec![];
        self.server.respond(
            req.success(ResponseBody::Scopes(ScopesResponse { scopes })),
        )?;
        Ok(())
    }

    fn handle_set_exceptions_breakpoints(&mut self, req: Request) -> Result<(), ServerError> {
        self.server.respond(req.success(ResponseBody::SetExceptionBreakpoints(
            SetExceptionBreakpointsResponse { breakpoints: None },
        )))?;
        Ok(())
    }

    fn handle_disassemble(&mut self, req: Request) -> Result<(), ServerError> {
        let Command::Disassemble(ref args) = req.command else {
            unreachable!("handle_disassemble called on a non disassemble request");
        };

        let instructions = vec![];

        self.server.respond(
            req.success(ResponseBody::Disassemble(DisassembleResponse { instructions })),
        )?;
        Ok(())
    }

    fn handle_step(&mut self, req: Request) -> Result<(), ServerError> {
        eprintln!("INFO: stepped by instruction");
        self.server.respond(req.ack()?)?;
        Ok(())
    }

    fn handle_next(&mut self, req: Request) -> Result<(), ServerError> {
        eprintln!("INFO: stepped by statement");
        self.server.respond(req.ack()?)?;
        Ok(())
    }

    fn handle_continue(&mut self, req: Request) -> Result<(), ServerError> {
        eprintln!("INFO: continue");
        self.server.respond(req.success(ResponseBody::Continue(ContinueResponse {
            all_threads_continued: Some(true),
        })))?;
        Ok(())
    }

    fn get_next_breakpoint_id(&mut self) -> i64 {
        let id = self.next_breakpoint_id;
        self.next_breakpoint_id += 1;
        id
    }

    fn handle_set_instruction_breakpoints(&mut self, req: Request) -> Result<(), ServerError> {
        let Command::SetInstructionBreakpoints(ref args) = req.command else {
            unreachable!("handle_set_instruction_breakpoints called on a different request");
        };

        let breakpoints = vec![];

        // response to request
        self.server.respond(req.success(ResponseBody::SetInstructionBreakpoints(
            SetInstructionBreakpointsResponse { breakpoints },
        )))?;
        Ok(())
    }

    fn handle_set_source_breakpoints(&mut self, req: Request) -> Result<(), ServerError> {
        let Command::SetBreakpoints(ref args) = req.command else {
            unreachable!("handle_set_source_breakpoints called on a different request");
        };

        let breakpoints = vec![];

        self.server.respond(
            req.success(ResponseBody::SetBreakpoints(SetBreakpointsResponse { breakpoints })),
        )?;
        Ok(())
    }
}

pub fn run_session<R: Read, W: Write>(
    server: Server<R, W>,
) -> Result<(), ServerError> {
    let mut session =
        DapSession::new(server);

    session.run_loop()
}