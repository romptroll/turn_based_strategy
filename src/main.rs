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

#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;  

use std::collections::HashMap;

mod net;
mod map;
mod scenes;
mod io;
mod camera;

use engine::{core::{info_log, input::{Input, Key, Mouse}, warn_log, window::{Action, Window}}, game::{Game, GameContainer, GameData}, renderer::{color::BLACK, graphics::Graphics, graphics3d::Graphics3D, renderer::{init_gl, std_renderer::{BlendMode, Capability, blend_func, enable}}, texture::{Texture, TextureRegion}}, scene::{Scene, SceneManager}};
use map::*;
use scenes::editor::*;
use net::{packet::*, client::Client, server::Server};
use io::resource::*;

type KeyEventDispatcher                 = bus::BusReader::<(Key, Action)>;
type MouseEventDispatcher               = bus::BusReader::<(Mouse, Action)>;
type MouseMoveEventDispatcher           = bus::BusReader::<(f32, f32)>;
type FrameBufferSizeEventDispatcher     = bus::BusReader::<(u32, u32)>;

struct TurnBasedStrategy {
    sm: SceneManager,
    win: Window,
    gfx: Graphics,
    inp: Input,
    key: KeyEventDispatcher,
    m:  MouseEventDispatcher,
    m_move: MouseMoveEventDispatcher,
    frame: FrameBufferSizeEventDispatcher,
    client: Client,
    timer: f32, 
}

/*impl TurnBasedStrategy {
    fn event(&mut self, gd: &mut GameData) -> bool {
        self.sm.event(event)
    }

    pub fn poll_peripheral_input(&mut self) {
        let mut loop_done = false;
        while !loop_done {
            match self.key.try_recv() {
                Ok(key_event) => {
                    self.event(EventHolder::new(key_event));
                },
                Err(_) => {
                    loop_done = true;
                }
            }
        }

        let mut loop_done = false;
        while !loop_done {
            match self.m.try_recv() {
                Ok(mouse_event) => {
                    self.event(EventHolder::new(mouse_event));
                },
                Err(_) => {
                    loop_done = true;
                }
            }
        }

        let mut loop_done = false;
        while !loop_done {
            match self.m_move.try_recv() {
                Ok(mouse_move_event) => {
                    self.event(EventHolder::new(mouse_move_event));
                },
                Err(_) => {
                    loop_done = true;
                }
            }
        }

        let mut loop_done = false;
        while !loop_done {
            match self.frame.try_recv() {
                Ok(frame_buffer_size_event) => {
                    self.event(EventHolder::new(frame_buffer_size_event));
                },
                Err(_) => {
                    loop_done = true;
                }
            }
        }
    }
}*/

impl Game for TurnBasedStrategy {
    fn on_start(&mut self, gd: &mut GameData) {
        self.sm.start(gd);
    }

    fn on_update(&mut self,  gd: &mut GameData) {
        //let mut current_scene = String::from(self.sm.get_current_scene_name());
        /*match self.sm.get_current_scene_mut() {
            Some(scene) =>  {
                let mut loop_done = false;
                while !loop_done {
                    match scene.events.pop() {
                        Some(event) => {
                            if event.is::<ChangeCurrentSceneEvent>() {
                                current_scene = event.downcast_ref::<ChangeCurrentSceneEvent>().unwrap().scene_name.clone();
                            }
                            if event.is::<ClientPacketEvent>() {
                                let packet = event.downcast_ref::<ClientPacketEvent>().unwrap().packet.clone();
                                self.client.send_data(packet);
                            }
                        }
                        None => {
                            loop_done = true;
                        }
                    }
                }
            }
            None => {},
        }*/

        //self.sm.set_current_scene(&current_scene);

        self.gfx.clear(BLACK);
        self.gfx.update();
        self.gfx.flush();

        //client_handler(self);

        self.sm.update(gd);
        self.sm.render(gd);

        self.timer += gd.delta_time();
        if self.timer > 1.0 {
            self.timer -= 1.0;
            self.win.set_title(&gd.frame_rate().to_string());
        }
        self.win.poll_events();
        self.win.swap_buffers();

        self.inp.update();
        //self.poll_peripheral_input();

        if self.win.should_close() {
            gd.shutdown();
        }
    }
}

/*fn load_game(game: &mut Game) {
    let loading_data = game.sm.get_current_scene_mut().unwrap();
    let loading_data = loading_data.em.get_all::<LoadingData>();
    for ld in loading_data {
        if ld.tag == "data" {

            //info_log!("{}\n/////////////////////////////////////", String::from_utf8(ld.data.clone()).unwrap());

            let data = Resource::from_string(&String::from_utf8(ld.data).unwrap());
            let atlas_path = data.get("atlas").get_index(0).get("path").as_str();
            let atlas = Texture::from_file(atlas_path);

            let mut textures = HashMap::new();

            for texture in data.get("textures").as_vec() {
                let texture_name = texture.get("name").as_str();
                let bounds = texture.get("bounds").as_vec();
                
                let w = bounds[2].as_i64() as u32;
                let h = bounds[3].as_i64() as u32;
                let x = bounds[0].as_i64() as u32;
                let y = atlas.height() - bounds[1].as_i64() as u32 - h;

                textures.insert(texture_name, TextureRegion::new(x, y, w, h, &atlas));
            }

            /*let mut game_scene = Scene::new("game");
            
            game_scene.add_system(Box::new(MapSystem {gfx: Graphics::new(&mut game.win)}));

            let mut tiles = Vec::new();
            tiles.push(game_scene.em.create_entity(Tile::new_invalid()));

            for tile in data.get("tiles").as_vec() {
                let r = match tile.get("r") {
                    Resource::F64(num) => *num, 
                    Resource::I64(num) => *num as f64, 
                    _ => 1.0,
                } as f32;
                let g = match tile.get("r") {
                    Resource::F64(num) => *num,
                    Resource::I64(num) => *num as f64,  
                    _ => 1.0,
                } as f32;
                let b = match tile.get("r") {
                    Resource::F64(num) => *num, 
                    Resource::I64(num) => *num as f64, 
                    _ => 1.0,
                } as f32;

                let texture = tile.get("texture").as_str();
                let texture = textures.get(texture).unwrap();
                tiles.push(game_scene.em.create_entity(Tile::new(r, g, b, texture.clone(), tiles.len() as u16)));
            }

            for map_data in data.get("map").as_vec() {
                let map = Map::from_file(map_data.get("path").as_str(), &tiles);
                game_scene.em.create_entity(map);
            }

            game_scene.em.create_entity(atlas);

            game.sm.add_scene(game_scene);*/
        }
    }    
}*/

/*fn start_game(server: &mut Server, players: &mut HashMap<std::net::SocketAddr, String>) {
    for (_k, name) in players.iter() {
        info_log!("{}", name);
    }
    println!();

    let data = Resource::from_file("res/data.yaml");
    let packet = Packet::new(PacketID::Data, data.to_string().as_bytes().to_vec());
    server.send_data(packet);

    std::thread::sleep_ms(100);

    for map_data in data.get("map").unwrap().as_vec() {
        let map = Resource::from_file(map_data.get("path").unwrap().as_str().unwrap());
        server.send_data(Packet::new(PacketID::Map, map.to_string().as_bytes().to_vec()));
    }

    std::thread::sleep_ms(10);

    server.send_data(Packet::new(PacketID::Start, Vec::new()));
}*/

fn server_handler() {
    let mut server = Server::new();
    let mut players = HashMap::new();

    loop {
        server.poll_new_client();
        
        for (p, addr) in server.poll_data() {
            match p.id {
                PacketID::Name => {
                    players.insert(addr, String::from(std::str::from_utf8(&p.data).unwrap()));
                },
                PacketID::Start => {
                    //start_game(&mut server, &mut players);
                },
                _ => {},
            }
        }
    }
}

/*#[derive(Clone)]
struct LoadingData {
    tag: String,
    data: Vec<u8>,
}

fn client_handler(game: &mut TurnBasedStrategy) {
    if game.sm.current_scene_name() == "loading" {
        match game.client.poll_data() {
            Some(packet) => {
                match packet.id {
                    PacketID::Data => {
                        let loading_data = LoadingData {
                            tag: String::from("data"),
                            data: packet.data,
                        };
                    },
                    PacketID::Map => {
                        let loading_data = LoadingData {
                            tag: String::from("map"),
                            data: packet.data,
                        };
                    },
                    PacketID::Start => {
                        //load_game(game);
                        game.sm.set_current_scene("game");
                    },
                    _ => {
                        warn_log!("Unknown packet with id: {} recived by client!", packet.id as u8);
                    },
                }
            },
            None => {},
        }
    }
}*/

fn main() {
    let mut win = Window::new(600, 400, "title: &str").unwrap();
    win.make_current();

    let inp = Input::new(&mut win);

    init_gl(&mut win);

    unsafe { enable(Capability::Blending); }
    unsafe { blend_func(BlendMode::SrcAlpha, BlendMode::OneMinusSrcAlpha); }

    let mut scene_manager = SceneManager::new();
    scene_manager.add_scene(Box::new(EditorScene::new(&mut win)), "editor");

    //scene_manager.add_scene(load_editor(&mut win));
    scene_manager.set_current_scene("editor");

    std::thread::spawn(move || {
        server_handler();
    });

    let client = Client::new();

    let game = TurnBasedStrategy {
        sm: scene_manager,
        gfx: Graphics::new(&mut win),
        inp: inp,
        key: win.create_key_listener(),
        m: win.create_mouse_listener(),
        m_move: win.create_mouse_move_listener(),
        frame: win.create_frame_buffer_listener(),
        win: win,
        client: client,
        timer: 0.0,
    };

    GameContainer::new().run(game);
}
