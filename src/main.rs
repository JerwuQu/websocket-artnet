use std::net::{TcpListener, UdpSocket};
use tungstenite::accept;

const WS_PORT: u16 = 9090;
const ART_PORT: u16 = 6454;
const ART_HEADER: &[u8] = b"Art-Net\x00";
const ART_OP_DMX: &[u8] = &[0x00, 0x50];

fn main() {
    let udp_sock = UdpSocket::bind(("127.0.0.1", 0)).expect("Failed to bind UDP");
    _ = udp_sock.set_broadcast(true);

    let ws_server = TcpListener::bind(("127.0.0.1", WS_PORT)).expect("Failed to bind TCP");
    println!("Listening on 127.0.0.1:{WS_PORT}");
    for stream in ws_server.incoming() {
        let mut ws = accept(stream.unwrap()).unwrap();
        println!("WebSocket client connected");
        let mut seq: u8 = 0;
        while let Ok(msg) = ws.read() {
            match msg {
                tungstenite::Message::Binary(buf) => {
                    if buf.len() != 514 || buf[0] != 0x01 {
                        println!("Client sent invalid packet. Disconnecting.");
                        break;
                    }
                    let uni = buf[1];
                    let dmx512 = &buf[2..2 + 512];

                    let mut buf = [0u8; 530];
                    buf[0..8].copy_from_slice(ART_HEADER);
                    buf[8..10].copy_from_slice(ART_OP_DMX);
                    buf[11] = 14;
                    buf[12] = seq;
                    buf[14] = uni;
                    buf[16] = 0x02;
                    buf[18..18 + 512].copy_from_slice(dmx512);
                    udp_sock
                        .send_to(&buf, ("127.0.0.1", ART_PORT))
                        .expect("Failed to send Art-Net packet");
                    seq = seq.wrapping_add(1);
                }
                tungstenite::Message::Text(_) => {
                    println!("Client sent text. Disconnecting.");
                    break;
                }
                tungstenite::Message::Close(_) => break,
                _ => {}
            }
        }
        println!("WebSocket client disconnected");
    }
}
