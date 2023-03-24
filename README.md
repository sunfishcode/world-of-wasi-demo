# World of WASI presentation demo

This is a demo showing WASI Preview2 sockets support. This is an early demo
using the APIs directly; in time we'll add standard library support so that
this isn't necessary.

## Running the demo

Clone the preview2-prototyping repository, which contains both the wit
IDL interfaces we'll use and the host implementation, and build the
host implementation:

```sh
$ git clone https://github.com/bytecodealliance/preview2-prototyping
[...]
$ cd preview2-prototyping
$ git checkout 408f0bf
[...]
$ cd host
$ cargo build
[...]
$ cd ../..
$ PATH="$PWD/preview2-prototyping/target/debug:$PATH"
```

This creates a command called `host` which includes Wasmtime and the
Preview2 host implementation.

Then build and start the server (adjusting the path to your local directory):

```sh
$ cd echo-server
$ cargo component build
[...]
    Creating component /home/demo/world-of-wasi-demo/echo-server/target/wasm32-wasi/debug/echo_server.wasm
$ host /home/demo/world-of-wasi-demo/echo-server/target/wasm32-wasi/debug/echo_server.wasm
Listening on IpSocketAddress::Ipv4(Ipv4SocketAddress { port: 42803, address: (127, 0, 0, 1) })
```

Note the port number that it prints out; we'll need to provide that to the client.

In another windows, build and run the client, passing it the port number:

```sh
$ cd echo-client
$ cargo component build
[...]
    Creating component /home/demo/world-of-wasi-demo/echo-client/target/wasm32-wasi/debug/echo_client.wasm
$ host /home/demo/world-of-wasi-demo/echo-client/target/wasm32-wasi/debug/echo_client.wasm 42803
Sending our message!
Receving the response!
Success!
```

If everything worked, the server will print this:

```
Accepted a connection!
Received input!
Sent output!
Received input!
Sent output!
The client closed the connection!
```

## How I created this repo

[Install Rust and protoc], and then [install cargo component], and then
create new projects for the echo-server and echo-client.

```sh
$ cargo-component new --lib echo-server
     Created component `echo-server` package
$ cargo-component new --lib echo-client
     Created component `echo-client` package
```

The use of `--lib` is a temporary workaround, though these will use the command
world and be runnable commands.

Delete the wit/world.wit file, and change the `package.metadata.component` in
echo-server/Cargo.toml to point to the wit in the preview2-prototyping repo we
checked out above:

```toml
[package.metadata.component]
target = { path = "../preview2-prototyping/wit", world = "command" }
```

Then put the code for the echo server and echo clients in echo-server/src/lib.rs
and echo-client/src/lib.rs.

[install Rust and protoc]: https://github.com/bytecodealliance/cargo-component#requirements
[install cargo component]: https://github.com/bytecodealliance/cargo-component#installation
