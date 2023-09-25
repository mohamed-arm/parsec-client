// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
//! Handler for TCP sockets
use super::{Connect, ReadWrite};
use crate::error::{ClientErrorKind, Error, Result};
use std::net::TcpStream;
use std::time::Duration;

/// Default socket on the server
pub const DEFAULT_SOCKET_PORT: u16 = 8000;

/// Default IP address
pub const DEFAULT_SOCKET_IP_ADDR: &str = "127.0.0.1";

/// IPC handler for TCP sockets
#[derive(Debug, Clone)]
pub struct TcpHandler {
    /// IP Address
    ip_addr: String,
    /// Port number
    port: u16,
    /// reads and writes on the streams
    timeout: Option<Duration>,
}

impl Connect for TcpHandler {
    fn connect(&self) -> Result<Box<dyn ReadWrite>> {
        let address = format!("{}:{}", self.ip_addr, self.port);
        let stream = TcpStream::connect(address).map_err(ClientErrorKind::Ipc)?;

        stream
            .set_read_timeout(self.timeout)
            .map_err(ClientErrorKind::Ipc)?;
        stream
            .set_write_timeout(self.timeout)
            .map_err(ClientErrorKind::Ipc)?;

        Ok(Box::from(stream))
    }

    fn set_timeout(&mut self, timeout: Option<Duration>) {
        self.timeout = timeout;
    }
}

impl TcpHandler {
    /// Create new client using given socket path and timeout duration
    pub fn new(ip_addr: String, port: u16, timeout: Option<Duration>) -> Result<Self> {
        if port > 1024 && port < 65535 {
            Ok(TcpHandler {
                ip_addr,
                port,
                timeout,
            })
        } else {
            Err(Error::Client(ClientErrorKind::InvalidSocketAddress))
        }
    }
}

impl Default for TcpHandler {
    fn default() -> Self {
        TcpHandler {
            ip_addr: DEFAULT_SOCKET_IP_ADDR.into(),
            port: DEFAULT_SOCKET_PORT.into(),
            timeout: Some(super::DEFAULT_TIMEOUT),
        }
    }
}
