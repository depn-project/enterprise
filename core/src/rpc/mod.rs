mod method;
mod middleware;

use jsonrpc_core::{Params, Result};
use jsonrpc_derive::rpc;
use jsonrpc_http_server::jsonrpc_core::MetaIoHandler;
use jsonrpc_http_server::ServerBuilder;
use middleware::{ValidationMiddleware, ValidationMiddlewareMeta};

#[rpc(server)]
pub trait CoreRPCMethods {
    #[rpc(name = "mode")]
    fn mode(&self) -> Result<crate::args::Mode>;
}

pub struct CoreRPC;

impl CoreRPCMethods for CoreRPC {
    fn mode(&self) -> Result<crate::args::Mode> {
        Ok(crate::args::Mode::Client)
    }
}

#[rpc(server)]
pub trait ClientRPCMethods {
    #[rpc(name = "test")]
    fn test(&self) -> Result<()>;

    #[rpc(name = "connect")]
    fn connect(&self) -> Result<()>;

    #[rpc(name = "disconnect")]
    fn disconnect(&self) -> Result<()>;
}

pub struct ClientRPC;

impl ClientRPCMethods for ClientRPC {
    fn test(&self) -> Result<()> {
        Ok(())
    }

    fn connect(&self) -> Result<()> {
        crate::vpn::VPN::connect();
        Ok(())
    }

    fn disconnect(&self) -> Result<()> {
        crate::vpn::VPN::disconnect();
        Ok(())
    }
}

#[rpc(server)]
pub trait GatewayRPCMethods {
    #[rpc(name = "test")]
    fn test(&self) -> Result<()>;
}

pub struct GatewayRPC;

impl GatewayRPCMethods for GatewayRPC {
    fn test(&self) -> Result<()> {
        Ok(())
    }
}

pub fn new() -> ServerBuilder<ValidationMiddlewareMeta, ValidationMiddleware> {
    let mut handler = MetaIoHandler::with_middleware(ValidationMiddleware::default());

    handler.add_method("vpn.disconnect", |_params: Params| async {
        Ok("hello".into())
    });

    ServerBuilder::new(handler)
}
