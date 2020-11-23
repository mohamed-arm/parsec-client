// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
//! Error types specific to the Parsec client
use parsec_interface::requests::ResponseStatus;
use std::error;
use std::fmt;

/// Enum used to denote errors returned to the library user
#[derive(Debug)]
pub enum Error {
    /// Errors originating in the service
    Service(ResponseStatus),
    /// Errors originating in the client
    Client(ClientErrorKind),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Service(response_status) => response_status.fmt(f),
            Error::Client(client_error_kind) => client_error_kind.fmt(f),
        }
    }
}
impl error::Error for Error {}

/// Types of errors local to the client library
#[derive(Debug)]
pub enum ClientErrorKind {
    /// Errors generated by the Parsec interface library
    Interface(ResponseStatus),
    /// Errors generated by interacting with the underlying IPC mechanism
    Ipc(::std::io::Error),
    /// The opcode of the response does not match the opcode of the request
    InvalidServiceResponseType,
    /// The operation is not supported by the selected provider
    InvalidProvider,
    /// Client is missing an implicit provider
    NoProvider,
    /// Service is missing authenticator or none of the authenticators is supported
    /// by the client
    NoAuthenticator,
    /// Required parameter was not provided
    MissingParam,
    /// Error while using the SPIFFE Workload API
    #[cfg(feature = "spiffe-auth")]
    Spiffe(spiffe::workload::Error),
}

impl From<ClientErrorKind> for Error {
    fn from(client_error: ClientErrorKind) -> Self {
        Error::Client(client_error)
    }
}

impl fmt::Display for ClientErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientErrorKind::Interface(response_status) => response_status.fmt(f),
            ClientErrorKind::Ipc(error) => error.fmt(f),
            ClientErrorKind::InvalidServiceResponseType => write!(
                f,
                "the opcode of the response does not match the opcode of the request"
            ),
            ClientErrorKind::InvalidProvider => {
                write!(f, "operation not supported by selected provider")
            }
            ClientErrorKind::NoProvider => write!(f, "client is missing an implicit provider"),
            ClientErrorKind::NoAuthenticator => write!(f, "service is not reporting any authenticators or none of the reported ones are supported by the client"),
            ClientErrorKind::MissingParam => write!(f, "one of the `Option` parameters was required but was not provided"),
            #[cfg(feature = "spiffe-auth")]
            ClientErrorKind::Spiffe(error) => error.fmt(f),
        }
    }
}

/// Result type used for the internals and interface of the Parsec client
pub type Result<T> = ::std::result::Result<T, Error>;
