use std::{net::{Ipv4Addr, TcpListener, TcpStream}, os::unix::net::SocketAddr};

use crate::config::ListenerConfig;



pub struct NetworkListener(TcpListener);



impl NetworkListener {
    pub fn new_network(config: &ListenerConfig)  -> Self{
        let i_love_listening = TcpListener::bind(format!("{}:{}", config.ip, config.port));
        assert!(i_love_listening.is_ok());
        Self(i_love_listening.unwrap())
    }

    pub fn port(&self) -> u16 {
        let port = self.0.local_addr().map(|addr| addr.port());
        if let Ok(m_port) = port {
            return m_port
        }
        unreachable!();
    }
}


impl Iterator for NetworkListener {
    type Item = TcpStream;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(stream) = self.0.incoming().next() && stream.is_ok(){
            return stream.ok();
        }
        None
    }
}


