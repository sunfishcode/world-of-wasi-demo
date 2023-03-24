use bindings::{filesystem, instance_network, network, streams, tcp, tcp_create_socket};

struct Command;

impl bindings::Command for Command {
    fn main(
        _stdin: streams::InputStream,
        _stdout: streams::OutputStream,
        _stderr: streams::OutputStream,
        _args: Vec<String>,
        _arg_preopens: Vec<(filesystem::Descriptor, String)>,
    ) -> Result<(), ()> {

        // Create a socket, bind it to localhost, listen, and accept a connection.

        let network = instance_network::instance_network();
        let socket = tcp_create_socket::create_tcp_socket(network::IpAddressFamily::Ipv4).unwrap();
        let address = network::IpSocketAddress::Ipv4(network::Ipv4SocketAddress {
            address: (127, 0, 0, 1),
            port: 0,
        });
        tcp::bind(socket, network, address).unwrap();
        tcp::listen(socket, network).unwrap();

        eprintln!("Listening on {:?}", tcp::local_address(socket).unwrap());

        let (_tcp_socket, input, output) = tcp::accept(socket).unwrap();

        eprintln!("Accepted a connection!");

        // Read data from the socket and echo it back.

        loop {
            let (data, eos) = streams::blocking_read(input, u64::MAX).unwrap();

            eprintln!("Received input!");

            let mut view = &data[..];
            while !view.is_empty() {
                let n = streams::blocking_write(output, &view).unwrap() as usize;
                view = &view[n..];
            }

            eprintln!("Sent output!");

            if eos {
                break;
            }
        }

        eprintln!("The client closed the connection!");

        Ok(())
    }
}

bindings::export!(Command);
