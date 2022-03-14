use std::{fmt::Debug, net::SocketAddr};

use color_eyre::{eyre::eyre, Report};
use knuffel::Decode;
use miette::{Diagnostic, MietteHandler, ReportHandler};

use crate::utils::Environment;

#[derive(Debug, Decode)]
struct KdlConfig {
    #[knuffel(child, unwrap(argument))]
    environment: Environment,

    #[knuffel(child)]
    frontend: Option<KdlFrontendConfig>,

    #[knuffel(child)]
    backend: Option<KdlBackendConfig>,
}

#[derive(Debug)]
pub struct Config {
    pub environment: Environment,
    pub frontend: Option<FrontendConfig>,
    pub backend: Option<BackendConfig>,
}

impl Config {
    pub fn parse(path: &str, text: &str) -> Result<Self, Report> {
        let config: KdlConfig = match knuffel::parse(path, text) {
            Ok(c) => c,
            Err(e) => {
                struct Wrapped<E>(E)
                where
                    E: Diagnostic;

                impl<E> std::fmt::Display for Wrapped<E>
                where
                    E: Diagnostic,
                {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        MietteHandler::new().debug(&self.0, f)
                    }
                }

                match e {
                    knuffel::Error::Syntax(e) => panic!("{}", Wrapped(e)),
                    knuffel::Error::Decode(e) => panic!("{}", Wrapped(e)),
                    _ => panic!("unknown error"),
                }
            }
        };
        config.try_into()
    }
}

impl TryFrom<KdlConfig> for Config {
    type Error = Report;

    fn try_from(v: KdlConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            environment: v.environment,
            frontend: v.frontend.map(TryInto::try_into).transpose()?,
            backend: v.backend.map(TryInto::try_into).transpose()?,
        })
    }
}

/// Configuration for the [frontend]
#[derive(Debug, Decode)]
struct KdlFrontendConfig {
    /// Address the frontend listens on (HTTP/1.1, TLS termination handled by others)
    #[knuffel(child, unwrap(argument, str))]
    listen: Option<SocketAddr>,

    /// Backend the frontend should talk to
    #[knuffel(child, unwrap(argument, str))]
    backend: Option<SocketAddr>,
}

#[derive(Debug)]
pub struct FrontendConfig {
    pub listen: SocketAddr,
    pub backend: SocketAddr,
}

impl TryFrom<KdlFrontendConfig> for FrontendConfig {
    type Error = Report;

    fn try_from(v: KdlFrontendConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            listen: v
                .listen
                .ok_or(eyre!("missing 'listen' in frontend config"))?,
            backend: v
                .backend
                .ok_or(eyre!("missing 'backend' in frontend config"))?,
        })
    }
}

/// Configuration for the [backend]
#[derive(Debug, Decode)]
struct KdlBackendConfig {
    /// Address the backend listens on (HTTP/1.1, unsecured, accessed over VPN)
    #[knuffel(child, unwrap(argument, str))]
    listen: Option<SocketAddr>,
}

#[derive(Debug)]
pub struct BackendConfig {
    pub listen: SocketAddr,
}

impl TryFrom<KdlBackendConfig> for BackendConfig {
    type Error = Report;

    fn try_from(v: KdlBackendConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            listen: v
                .listen
                .ok_or(eyre!("missing 'address' in backend config"))?,
        })
    }
}

#[derive(Debug)]
pub struct KdlSocketAddr(SocketAddr);

impl From<KdlSocketAddr> for SocketAddr {
    fn from(sa: KdlSocketAddr) -> Self {
        sa.0
    }
}
