pub struct TcpTransport {}

impl TcpTransport {
    
}

// pub struct WsTransport {

// }

pub enum Transport {
    Tcp(TcpTransport),
    // Ws(WsTransport),
}
