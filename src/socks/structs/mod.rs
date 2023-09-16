use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug)]
pub enum SocksCMD {
    Connect,
    Bind,
    UdpAssociate,
}

#[derive(Debug)]
pub enum SocksAddressType {
    IPv4,
    Domain,
    IPv6,
}

pub struct Connection {
    pub(crate) buf: [u8; 1024],
    pub(crate) stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            buf: [0u8; 1024],
            stream,
        }
    }

    pub async fn read(&mut self) -> Result<Vec<u8>, String> {
        let data_length = self.stream.read(&mut self.buf).await
            .map_err(|err| err.to_string())?;
        let data = self.buf[..data_length].to_vec();
        Ok(data)
    }

    pub async fn write(&mut self, data: &[u8]) -> Result<(), String> {
        self.stream.write(data).await
            .map_err(|err| err.to_string())?;
        self.stream.flush().await
            .map_err(|err| err.to_string())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct HandShakeRequest {
    pub(crate) version: u8,
    pub(crate) methods: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum AuthMethod {
    NoAuth,
    GSSAPI,
    UsernamePassword,
    IANAAssigned,
    Reserved,
    NoAcceptableMethods,
}

#[derive(Debug, Clone)]
pub struct HandShakeResponse {
    pub(crate) version: u8,
    pub(crate) method: u8,
}

impl HandShakeResponse {
    pub fn new(auth_method: AuthMethod) -> Self {
        Self {
            version: 0x05u8,
            method: 0x00u8,
        }
    }
}