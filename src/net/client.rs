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

use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use std::option::Option;

use engine::core::error_log;

use crate::net::packet;

pub struct Client {
    server: TcpStream,
    big_packet: packet::BigPacket,
}

impl Client {
    pub fn new() -> Client {
        let server = TcpStream::connect("127.0.0.1:8000").unwrap();
        server.set_nonblocking(true).unwrap();
        
        Client {
            server,
            big_packet: packet::BigPacket::new(),
        }
    }

    pub fn poll_data(&mut self) -> Option<packet::Packet> {
        let mut buff = [0; packet::MAX_PACKED_SIZE];
        
        match self.server.read(&mut buff) {
            Ok(buff_len) => {
                let packet = packet::Packet::from_raw(&buff, buff_len);
                if packet.is_none() {
                    return None;
                }

                let packet = packet.unwrap();

                match packet.id {
                    packet::PacketID::Multiple => {
                       self.big_packet.add(packet)
                    },
                    _ => {
                        Some(packet)
                    }
                }
            },
            Err(_e) => {
                None
            },
        }
    }

    fn send_single_packet(&mut self, packet: packet::Packet) {
        let mut packet = packet;
        let mut data = vec!(packet.id as u8);
        data.append(&mut packet.data);
        
        match self.server.write_all(&data) {
            Ok(_) => {},
            Err(e) => {
                error_log!("{}", e);
            },
        }
    }

    pub fn send_data(&mut self, packet: packet::Packet) {
        if packet.data.len() < packet::MAX_PACKED_SIZE - 4 {
            self.send_single_packet(packet);
        }
        else {
            let packets = packet::Packet::create_multiple(packet);
            for packet in packets {
                self.send_single_packet(packet);
            }
        }
    }
}