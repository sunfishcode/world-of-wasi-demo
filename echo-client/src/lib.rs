use bindings::{filesystem, instance_network, network, streams, tcp, tcp_create_socket, exit};
use std::str::FromStr;

struct Command;

impl bindings::Command for Command {
    fn main(
        _stdin: streams::InputStream,
        _stdout: streams::OutputStream,
        _stderr: streams::OutputStream,
        args: Vec<String>,
        _arg_preopens: Vec<(filesystem::Descriptor, String)>,
    ) -> Result<(), ()> {

        if args.len() != 2 {
            eprintln!("usage: echo-client <port>");
            exit::exit(Err(()));
        }
        let port = u16::from_str(&args[1]).unwrap();

        // Create a socket, bind it to localhost, listen, and accept a connection.

        let network = instance_network::instance_network();
        let socket = tcp_create_socket::create_tcp_socket(network::IpAddressFamily::Ipv4).unwrap();
        let address = network::IpSocketAddress::Ipv4(network::Ipv4SocketAddress {
            address: (127, 0, 0, 1),
            port,
        });
        let (input, output) = tcp::connect(socket, network, address).unwrap();

        eprintln!("Sending our message!");

        let message = "Hello, world!".to_owned().into_bytes();
        let mut view = &message[..];
        while !view.is_empty() {
            let n = streams::blocking_write(output, &view).unwrap() as usize;
            view = &view[n..];
        }

        eprintln!("Receving the response!");

        // Read data from the socket and echo it back.

        let mut response = Vec::new();
        while response.len() < message.len() {
            let (data, eos) = streams::blocking_read(input, u64::MAX).unwrap();
            response.extend_from_slice(&data);
            if eos {
                break;
            }
        }

        assert_eq!(response, message);

        eprintln!("Success!");

        Ok(())
    }
}

bindings::export!(Command);
