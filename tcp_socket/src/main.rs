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
use std::collections::HashMap;
use std::net::Ipv4Addr;

mod tcp;

#![derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
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
            Ok(packet) => {
                let src = packet.source_addr();
                let dest = packet.destination_addr();
                let protocol = packet.protocol();

                /*
                 * Not TCP
                 */
                if protocol != etherparse::IpNumber(0x06) {
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + packet.slice().len()..]) {
                    Ok(packet) => {
                        connections.entry(Quad {
                            src: (src, packet.source_port()),
                            dst: (dest, packet.destination_port()),
                        }).or_default();

                        eprintln!(
                            "{} -> {} {}b of TCP Packet to port {}",
                            src,
                            dest,
                            packet.slice().len(),
                            packet.destination_port(),
                        );
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
