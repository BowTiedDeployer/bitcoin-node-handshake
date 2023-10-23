# Peer to peer node handshake on Bitcoin

## Handling the Handshake

The handshake process is initiated by connecting to a target node, sending a version message, and expecting a version message in return. Upon successful version exchange, a verack message is sent to the node, and a verack message is expected in return to complete the handshake. [wiki link here](https://en.bitcoin.it/wiki/Version_Handshake)

## How to run

1. Clone this repository:

```bash
git clone <repository_url>
cd <repository_name>
```

2. Build the project:

```bash
cargo build --release
```

3. Run the CLI with desired commands, for example (optional argument `-t` for testnet):

- To list available DNS seeds:

```bash
cargo run -- -dns [-t]
```

- To fetch IPs from specified DNS seeds (optional argument `-t` for testnet):

```bash
cargo run -- -ips [-t] <number> <DNS> <DNS> ... <DNS>
```

- To perform a handshake with specified IPs and the optional arguments:
  - `-t` for testnet
  - `-f` to log the output in `output.log` instead of the terminal

```bash
cargo run -- -handshake [-t] [-f] <IP> <IP> ... <IP>
```

## How to test the Handshake

After running the handshake command, check the console (or file) output to verify that the version and verack messages were successfully exchanged with the target node(s).
