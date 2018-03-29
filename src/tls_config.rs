extern crate native_tls;

use errors::*;
use std::fmt;
use self::native_tls::{Protocol, TlsConnector, TlsConnectorBuilder as InternalBuilder};
use self::native_tls::Certificate;
use self::native_tls::Pkcs12;

#[derive(Clone)]
pub struct TlsConfig(TlsConnector);

pub struct TlsConfigBuilder(InternalBuilder);

impl TlsConfigBuilder {
    pub fn new() -> Result<TlsConfigBuilder, NatsError> {
        Ok(TlsConnector::builder().map(|x| TlsConfigBuilder(x))?)
    }

    pub fn add_root_certificate(&mut self, cert: Certificate) -> Result<&mut Self, NatsError> {
        self.0.add_root_certificate(cert);
        Ok(self)
    }

    pub fn add_client_certificate(
        &mut self,
        cert: Certificate,
        key: Pkcs12,
    ) -> Result<&mut Self, NatsError> {
        {
            let ctx = &mut self.0;
            ctx.add_root_certificate(cert)?;
            ctx.identity(key)?;
            //            ctx.check_private_key()?;
        }
        Ok(self)
    }

    pub fn build(self) -> Result<TlsConfig, NatsError> {
        Ok(self.0.build().map(|x| TlsConfig(x))?)
    }
}

impl TlsConfig {
    pub fn into_connector(self) -> TlsConnector {
        self.0
    }
}

impl fmt::Debug for TlsConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TlsConfig {{}}")
    }
}
