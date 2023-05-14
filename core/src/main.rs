mod args;
mod rpc;
mod vpn;

use args::{DepnCoreArgs, Mode};
use clap::Parser;
use jsonrpc_core::IoHandler;
use jsonrpc_http_server::ServerBuilder;
use rpc::{CoreRPC, CoreRPCMethods, ClientRPC, ClientRPCMethods};
use std::thread;

#[cfg(not(target_family = "windows"))]
fn run_gateway_rpc() -> std::thread::JoinHandle<()>{
    thread::spawn(|| {
        todo!()
    })
}

fn run_client_rpc() -> std::thread::JoinHandle<()> {
    thread::spawn(|| {
        let mut io = IoHandler::new();
        io.extend_with(ClientRPC.to_delegate());

        let core_server = ServerBuilder::new(io)
            .start_http(&"127.0.0.1:3031".parse().unwrap())
            .unwrap();

        println!("Client RPC run on 127.0.0.1:3031");

        core_server.wait();
    })
}

fn main() {
    let args = DepnCoreArgs::parse();
    let mode: Mode = args.mode;

    let mode_rpc_handle = match mode {
        #[cfg(not(target_family = "windows"))]
        Mode::Gateway => run_gateway_rpc(),
        Mode::Client => run_client_rpc(),
    };

    let mut core_io = IoHandler::new();
    core_io.extend_with(CoreRPC.to_delegate());

    let core_server = ServerBuilder::new(core_io)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();

    println!("Core RPC run on 127.0.0.1:3030");

    core_server.wait();
    mode_rpc_handle.join().unwrap();
}
