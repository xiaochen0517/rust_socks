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