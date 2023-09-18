use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug)]
pub enum SocksCMD {
    Connect,
    Bind,
    UdpAssociate,
}

pub fn get_socks_cmd(cmd: &u8) -> Result<SocksCMD, String> {
    return match *cmd {
        0x01u8 => {
            Ok(SocksCMD::Connect)
        }
        0x02u8 => {
            Ok(SocksCMD::Bind)
        }
        0x03u8 => {
            Ok(SocksCMD::UdpAssociate)
        }
        _ => {
            Err("不支持的命令".to_string())
        }
    };
}

#[derive(Debug)]
pub enum SocksAddressType {
    IPv4,
    Domain,
    IPv6,
}

pub fn get_socks_address_type(types: &u8) -> Result<SocksAddressType, String> {
    return match *types {
        0x01u8 => {
            Ok(SocksAddressType::IPv4)
        }
        0x03u8 => {
            Ok(SocksAddressType::Domain)
        }
        0x04u8 => {
            Ok(SocksAddressType::IPv6)
        }
        _ => {
            Err("不支持的地址类型".to_string())
        }
    };
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

#[derive(Debug)]
pub struct ForwardRequest {
    pub(crate) version: u8,
    pub(crate) cmd: SocksCMD,
    pub(crate) address_type: SocksAddressType,
    pub(crate) host: Vec<u8>,
    pub(crate) port: Vec<u8>,
    pub(crate) address: String,
}