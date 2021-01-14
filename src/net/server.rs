/*
 *   Copyright (c) 2020 Ludwig Bogsveen
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

use std::net::TcpListener;
use std::net::TcpStream;
use std::net::SocketAddr;
use std::vec::Vec;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;

use engine::core::error_log;

use crate::net::packet;

pub struct Server {
    listener: TcpListener,
    clients: Vec<(TcpStream, SocketAddr)>,
    big_packets: HashMap<SocketAddr, packet::BigPacket>,
}

impl Server {
    pub fn new() -> Server {
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
        listener.set_nonblocking(true).unwrap();

        Server {
            listener,
            clients: Vec::new(),
            big_packets: HashMap::new(),
        }
    }

    pub fn poll_new_client(&mut self) {
        let mut loop_done = false;
        while !loop_done {
            match self.listener.accept() {
                Ok(stream) => {
                    self.big_packets.insert(stream.1, packet::BigPacket::new());
                    self.clients.push(stream);
                },
                Err(_e) => {
                    loop_done = true;
                }
            }
        }
    }

    pub fn poll_data(&mut self) -> Option<(packet::Packet, SocketAddr)> {
        for (stream, addr) in &mut self.clients {
            let mut buff = [0; packet::MAX_PACKED_SIZE];
            match stream.read(&mut buff) {
                Ok(buff_len) => {
                    let packet = packet::Packet::from_raw(&buff, buff_len);
                    if packet.is_none() {
                        continue;
                    }
    
                    let packet = packet.unwrap();

                    match packet.id {
                        packet::PacketID::Multiple => {    
                            let big_packet = self.big_packets.get_mut(addr).unwrap();
                            let packet = big_packet.add(packet);
                            match packet {
                                Some(packet) => {
                                    return Some((packet, addr.clone()));
                                },
                                None => {
                                    return None;
                                }
                            }
                        },
                        _ => {
                            return Some((packet, addr.clone()));
                        }
                    }
                },
                Err(_e) => {},
            }
        }
        None
    }

    fn send_single_packet(&mut self, packet: packet::Packet) {
        let mut packet = packet;
        let mut data = vec!(packet.id as u8);
        data.append(&mut packet.data);

        for (stream, _addr) in &mut self.clients {    
            match stream.write_all(&data) {
                Ok(_) => {},
                Err(e) => {
                    error_log!("{}", e);
                },
            }
        }
    }

    pub fn send_data(&mut self, packet: packet::Packet) {
        if packet.data.len() < packet::MAX_PACKED_SIZE - 4 {
            self.send_single_packet(packet);
        }
        else {
            //info_log!("\n{}", String::from_utf8(packet.data.clone()).unwrap());
            let packets = packet::Packet::create_multiple(packet);
            for packet in packets {
                std::thread::sleep_ms(1);
                self.send_single_packet(packet);
            }
        }
    }

    pub fn num_clients(&self) -> usize {
        self.clients.len()
    }
}