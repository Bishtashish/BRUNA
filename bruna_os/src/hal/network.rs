// bruna_os/src/hal/network.rs
use super::common::{HalResult, HardwareId};

#[derive(Debug, Clone)]
pub enum IpAddress {
    V4([u8; 4]),
    V6([u16; 8]),
}

pub trait NetworkInterface {
    fn new(interface_name: &str) -> HalResult<Self> where Self: Sized;
    fn get_id(&self) -> HardwareId; // e.g., MAC address
    fn get_ip_address(&self) -> HalResult<IpAddress>;
    fn send(&mut self, data: &[u8], destination_ip: IpAddress, port: u16) -> HalResult<()>;
    fn receive(&mut self, buffer: &mut [u8]) -> HalResult<(usize, IpAddress, u16)>; // (bytes_read, source_ip, source_port)
    // fn connect_tcp(destination_ip: IpAddress, port: u16) -> HalResult<TcpStream>;
    // fn listen_udp(port: u16) -> HalResult<UdpSocket>;
}
