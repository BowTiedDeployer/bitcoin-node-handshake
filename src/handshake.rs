use crate::config::CliNetwork;
use crate::network::{get_verack_message, get_version_message};
use bitcoin::consensus::Decodable;
use bitcoin::network::message::{NetworkMessage, RawNetworkMessage};
use std::io::{BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::ops::Add;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

#[derive(thiserror::Error, Debug)]
pub enum HandshakeError {
    #[error("Failed to clone stream: {0}")]
    StreamCloneError(String),
    #[error("Failed to retrieve address: {0}")]
    AddressRetrieveError(String),
    #[error("Failed to send version message: {0}")]
    VersionSendError(String),
    #[error("Failed to send verack message: {0}")]
    VerackSendError(String),
    #[error("Failed to decode version message from remote: {0}")]
    VersionDecodeError(String),
    #[error("Failed to decode verack message from remote: {0}")]
    VerackDecodeError(String),
    #[error("emote hasn't sent a verack message back!")]
    NoVerackResponse,
    #[error("Remote hasn't sent a version message back!")]
    NoVersionResponse,
    #[error("Failed to connect to node: {0}")]
    NodeConnectionError(String),
    #[error("Error running handshake: {0}")]
    HandshakeRuntimeError(String),
    #[error("Failed to join handshake thread: {0}")]
    JoinHandleError(String),
    #[error("Handshake timed out.")]
    HandshakeTimeout,
}

pub async fn run_handshake(
    ip_list: Vec<SocketAddr>,
    print_in_file: bool,
    network: Arc<CliNetwork>,
) {
    let mut output_writer: Box<dyn Write> = if print_in_file {
        match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("output.log")
        {
            Ok(file) => {
                if let Err(e) = file.set_len(0) {
                    println!("Failed to clear file contents: {}", e);
                }
                Box::new(file)
            }
            Err(e) => {
                println!("Failed to open output.log: {}", e);
                return;
            }
        }
    } else {
        Box::new(std::io::stdout())
    };

    for ip in ip_list {
        log_message(
            &mut output_writer,
            format!("Performing handshake for {:?}:", &ip),
        );
        let network_arc = Arc::clone(&network);

        let handshake =
            tokio::spawn(async move { do_handshake(ip, print_in_file.clone(), network_arc).await });

        let timeout = timeout(Duration::from_secs(4), handshake).await;

        let timeout_result = match timeout {
            Ok(result) => match result {
                Ok(handshake) => match handshake {
                    Ok(()) => Ok(()),
                    Err(e) => Err(HandshakeError::HandshakeRuntimeError(e.to_string())),
                },
                Err(e) => Err(HandshakeError::JoinHandleError(e.to_string())),
            },
            Err(_) => Err(HandshakeError::HandshakeTimeout),
        };

        if let Err(e) = timeout_result {
            log_message(&mut output_writer, e.to_string().add("\n"));
        }
    }
}

fn log_message(output_writer: &mut Box<dyn Write>, message: String) {
    if let Err(err) = writeln!(output_writer, "{}", message) {
        println!("Failed to log message: {}", err);
    }
}

async fn do_handshake(
    ip: SocketAddr,
    print_in_file: bool,
    network: Arc<CliNetwork>,
) -> Result<(), HandshakeError> {
    let mut log_writer: Box<dyn Write> = if print_in_file {
        match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("output.log")
        {
            Ok(file) => Box::new(file),
            Err(e) => {
                println!("Failed to open output.log: {}", e);
                Box::new(std::io::stdout())
            }
        }
    } else {
        Box::new(std::io::stdout())
    };

    return match TcpStream::connect(ip) {
        Ok(mut stream) => {
            let read = match stream.try_clone() {
                Ok(stream) => stream,
                Err(e) => return Err(HandshakeError::StreamCloneError(e.to_string())),
            };

            let mut buf_reader = BufReader::new(read);

            let local_address = match stream.local_addr() {
                Ok(address) => address,
                Err(e) => {
                    return Err(HandshakeError::AddressRetrieveError(e.to_string()));
                }
            };

            let remote_address = match stream.peer_addr() {
                Ok(address) => address,
                Err(e) => {
                    return Err(HandshakeError::AddressRetrieveError(e.to_string()));
                }
            };

            let version_message_local =
                get_version_message(local_address, remote_address, &network);

            if let Err(e) = stream.write_all(version_message_local.as_slice()) {
                return Err(HandshakeError::VersionSendError(e.to_string()));
            };

            log_message(
                &mut log_writer,
                format!("Sent version message to {:?}!", remote_address),
            );

            match RawNetworkMessage::consensus_decode(&mut buf_reader) {
                Ok(decoded_message) => match decoded_message.payload {
                    NetworkMessage::Version(message) => {
                        log_message(
                            &mut log_writer,
                            format!("Got version message back: {}", message.version),
                        );

                        let verack_message_local = get_verack_message(&network);

                        if let Err(e) = stream.write_all(verack_message_local.as_slice()) {
                            return Err(HandshakeError::VerackSendError(e.to_string()));
                        };

                        log_message(
                            &mut log_writer,
                            format!("Sent verack message to {:?}!", remote_address),
                        );

                        match RawNetworkMessage::consensus_decode(&mut buf_reader) {
                            Ok(decoded_message) => match decoded_message.payload {
                                NetworkMessage::Verack => {
                                    log_message(
                                        &mut log_writer,
                                        format!("Got verack message back.\n"),
                                    );
                                    Ok(())
                                }
                                _ => Err(HandshakeError::NoVerackResponse),
                            },
                            Err(e) => Err(HandshakeError::VerackDecodeError(e.to_string())),
                        }
                    }
                    _ => Err(HandshakeError::NoVersionResponse),
                },
                Err(e) => Err(HandshakeError::VersionDecodeError(e.to_string())),
            }
        }
        Err(e) => Err(HandshakeError::NodeConnectionError(e.to_string())),
    };
}
