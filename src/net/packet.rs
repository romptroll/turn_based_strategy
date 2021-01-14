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

use std::vec::Vec;

#[derive(Clone, Copy)]
pub struct MultiplePacketHeader {
    size: u8,
    index: u8,
    id: PacketID,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum PacketID {
    None,
    Name,
    Start,
    Multiple,
    Data,
    Map,
}

pub const MULTIPLE_PACKET_HEADER_SIZE: usize = 4; // Multiple id, Num of packets, Current package index, Packet ID
pub const MAX_PACKED_SIZE: usize = 256;

#[derive(Clone)]
pub struct Packet {
    pub id: PacketID,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn new(id: PacketID, data: Vec<u8>) -> Packet {
        let mut data = data;
        if data.len() == 0 {
            data.push(0);
        }
        Packet {
            id,
            data,
        }
    }

    pub fn create_header(len: u8, index: u8, id: PacketID) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(len);
        data.push(index);
        data.push(id as u8);
        data
    }
    
    pub fn create_multiple(packet: Packet) -> Vec<Packet> {
        let mut packets = Vec::new();

        let num_packets = (packet.data.len() as f32 / (MAX_PACKED_SIZE - MULTIPLE_PACKET_HEADER_SIZE) as f32).ceil() as u8;
        let mut index = 0;

        let mut data_piece = Packet::create_header(num_packets, packets.len() as u8, packet.id);

        for _ in 0..num_packets {
            for _ in 0..(MAX_PACKED_SIZE - MULTIPLE_PACKET_HEADER_SIZE) {
                if index >= packet.data.len() {
                    break;
                }
                data_piece.push(packet.data[index]);
                index += 1;
            }

            packets.push(Packet::new(PacketID::Multiple, data_piece));

            data_piece = Packet::create_header(num_packets, packets.len() as u8, packet.id);
        }
        packets
    }

    pub fn from_multiple(packets: &Vec<Packet>) -> Packet {
        let mut data = Vec::new();

        for i in 0..packets.len() {
            for j in 0..packets.len() {
                if i == packets[j].data[1] as usize {
                    for l in 3..packets[j].data.len() {
                        data.push(packets[j].data[l]);
                    }
                }
            }
        }

        Packet::new(unsafe { std::mem::transmute(packets[0].data[2]) }, data)
    }

    pub fn from_raw(buff: &[u8], len: usize) -> Option<Packet> {
        if len == 0 {
            return None
        }
        
        let id = unsafe { std::mem::transmute(buff[0]) };
        let mut data = Vec::with_capacity(len - 1);

        for i in 1..len {
            data.push(buff[i]);
        }

        Some(Packet::new(id, data))
    }
}

pub struct BigPacket {
    packets: Vec<Packet>,
}

impl BigPacket {
    pub fn new() -> BigPacket {
        BigPacket {
            packets: Vec::new(),
        }
    }

    pub fn add(&mut self, packet: Packet) -> Option<Packet> {
        let num_packets = packet.data[0] as usize;

        self.packets.push(packet);

        if self.packets.len() == num_packets {
            let p = Packet::from_multiple(&self.packets);
            self.packets.clear();
            return Some(p);
        }
        else {
            None
        }
    }
}