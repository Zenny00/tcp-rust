pub enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,
}

impl Default for State {
    fn default() -> Self {
        // State::Closed
        State::Listen
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        ip_header: etherparse::Ipv4HeaderSlice<'a>,
        tcp_header: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
        let mut buf = [0u8, 1500];
        match *self {
            State::Closed => {
                return;
            }
            State::Listen => {
                if !tcp_header.syn() {
                    // Only expected syn packet
                    return;
                }

                // Establish a connection
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcp_header.destination_port(),
                    tcp_header.source_port(),
                    0,
                    0,
                );
                syn_ack.syn = true;
                syn_ack.ack = true;

                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len_u16(),
                    64,
                    etherparse::IpNumber::TCP,
                    ip_header.destination_addr(),
                    ip_header.source_addr(),
                );

                let unwritten = {
                    let mut unwritten = &mut buf[..];
                    ip.write(unwritten);
                    syn_ack.write(unwritten);
                };
                nic.send(&buf[..unwritten])
            }
            State::Estab => {
                return;
            }
            State::SynRcvd => {
                return;
            }
        }
        eprintln!(
            "{}:{} -> {}:{} {}b of TCP Packet",
            ip_header.source_addr(),
            tcp_header.source_port(),
            ip_header.destination_addr(),
            tcp_header.destination_port(),
            data.len(),
        );
    }
}
