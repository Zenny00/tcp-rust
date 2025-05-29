use std::collections::HashMap;
/**
* The goal of this project is to build a basic implementation of
* the Transmission Control Protocol (TCP) as specified in RFC 793 (https://www.rfc-editor.org/rfc/rfc793)
*
* For the implementation I followed along with Jon Gjengset's YouTube playlist
* of him doing the same (https://www.youtube.com/watch?v=bzja9fQWzdA&list=PLqbS7AVVErFivDY3iKAQk3_VAm8SXwt1X)
*
* My hope is to gain a better understanding of how the protocol works while also getting a better
* understand of the Rust language.
*/
use std::io;
use std::net::Ipv4Addr;

mod tcp;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, tcp::State> = Default::default();

    // Create a new tun with the name "tun0"
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("Failed to create tunnel");
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let _flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);

        /*
         * Ignore any packet that is not IPv4
         */
        if proto != 0x800 {
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(ip_header) => {
                let src = ip_header.source_addr();
                let dest = ip_header.destination_addr();
                let protocol = ip_header.protocol();

                /*
                 * Not TCP
                 */
                if protocol != etherparse::IpNumber(0x06) {
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(
                    &buf[4 + ip_header.slice().len()..nbytes],
                ) {
                    Ok(tcp_header) => {
                        let data_index = 4 + ip_header.slice().len() + tcp_header.slice().len();
                        connections
                            .entry(Quad {
                                src: (src, tcp_header.source_port()),
                                dst: (dest, tcp_header.destination_port()),
                            })
                            .or_default()
                            .on_packet(ip_header, tcp_header, &buf[data_index..nbytes]);
                    }
                    Err(err) => {
                        eprintln!("Ignoring incorrect format packet {:?}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Ignoring incorrect format packet {:?}", err);
            }
        }
    }
}
