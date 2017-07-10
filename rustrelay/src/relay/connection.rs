use std::fmt;
use std::net::{Ipv4Addr, SocketAddrV4};

use super::ipv4_header::{IPv4HeaderData, Protocol};
use super::ipv4_packet::IPv4Packet;
use super::net;
use super::selector::Selector;
use super::transport_header::TransportHeaderData;

const LOCALHOST_FORWARD: u32 = 0x0A000202;

pub trait Connection {
    fn id(&self) -> &ConnectionId;
    fn send_to_network(&mut self, selector: &mut Selector, ipv4_packet: &IPv4Packet);
    fn close(&mut self, selector: &mut Selector);
    fn is_expired(&self) -> bool;
    fn is_closed(&self) -> bool;
}

pub fn rewritten_destination(ipv4: u32, port: u16) -> SocketAddrV4  {
    let addr = if ipv4 == LOCALHOST_FORWARD {
        Ipv4Addr::new(127, 0, 0, 1)
    } else {
        net::to_addr(ipv4)
    };
    SocketAddrV4::new(addr, port)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionId {
    protocol: Protocol,
    source_ip: u32,
    source_port: u16,
    destination_ip: u32,
    destination_port: u16,
}

impl ConnectionId {
    pub fn from_headers(ipv4_header_data: &IPv4HeaderData, transport_header_data: &TransportHeaderData) -> Self {
        Self {
            protocol: ipv4_header_data.protocol(),
            source_ip: ipv4_header_data.source(),
            source_port: transport_header_data.source_port(),
            destination_ip: ipv4_header_data.destination(),
            destination_port: transport_header_data.destination_port(),
        }
    }

    pub fn protocol(&self) -> Protocol {
        self.protocol
    }

    pub fn source_ip(&self) -> u32 {
        self.source_ip
    }

    pub fn source_port(&self) -> u16 {
        self.source_port
    }

    pub fn destination_ip(&self) -> u32 {
        self.destination_ip
    }

    pub fn destination_port(&self) -> u16 {
        self.destination_port
    }

    pub fn source(&self) -> SocketAddrV4 {
        net::to_socket_addr(self.source_ip, self.source_port)
    }

    pub fn destination(&self) -> SocketAddrV4 {
        net::to_socket_addr(self.destination_ip, self.destination_port)
    }
}

impl fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.source(), self.destination())
    }
}

// macros to log connection id along with the message

macro_rules! cx_format {
    ($id:tt, $str:tt, $($arg:tt)+) => {
        format!(concat!("{} ", $str), $id, $($arg)+)
    };
    ($id:tt, $str:tt) => {
        format!(concat!("{} ", $str), $id)
    };
}

macro_rules! cx_trace {
    (target: $target:expr, $id:expr, $($arg:tt)*) => {
        trace!(target: $target, "{}", cx_format!($id, $($arg)+));
    }
}

macro_rules! cx_debug {
    (target: $target:expr, $id:expr, $($arg:tt)*) => {
        debug!(target: $target, "{}", cx_format!($id, $($arg)+));
    }
}

macro_rules! cx_info {
    (target: $target:expr, $id:expr, $($arg:tt)*) => {
        info!(target: $target, "{}", cx_format!($id, $($arg)+));
    }
}

macro_rules! cx_warn {
    (target: $target:expr, $id:expr, $($arg:tt)*) => {
        warn!(target: $target, "{}", cx_format!($id, $($arg)+));
    }
}

macro_rules! cx_error {
    (target: $target:expr, $id:expr, $($arg:tt)*) => {
        error!(target: $target, "{}", cx_format!($id, $($arg)+));
    }
}
