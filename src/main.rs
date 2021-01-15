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



mod net;
mod map;
mod scenes;
mod io;
mod camera;
mod entities;

use engine::{core::{input::{Input, Key, Mouse}, window::{Action, Window}}, game::{Game, GameContainer, GameData}, renderer::{color::BLACK, graphics::Graphics, renderer::{init_gl, std_renderer::{BlendMode, Capability, blend_func, enable}}}, scene::{SceneManager}};

use scenes::{editor::*, game::GameScene, main_menu::MainMenuScene};
use net::{client::{Client, ClientHandler}, packet::*, server::{Server, ServerManager}};


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
    client: ClientHandler,
    timer: f32, 
}


impl Game for TurnBasedStrategy {
    fn on_start(&mut self, gd: &mut GameData) {
        self.sm.start(gd);
        //self.client.send_name("Ludwig");
    }

    fn on_update(&mut self,  gd: &mut GameData) {
        self.client.update();

        self.gfx.clear(BLACK);
        self.gfx.update();
        self.gfx.flush();

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

        if self.win.should_close() {
            gd.shutdown();
        }
    }
}

fn main() {
    let mut win = Window::new(600, 400, "title: &str").unwrap();
    win.make_current();

    let inp = Input::new(&mut win);

    init_gl(&mut win);

    unsafe { enable(Capability::Blending); }
    unsafe { blend_func(BlendMode::SrcAlpha, BlendMode::OneMinusSrcAlpha); }

    let mut scene_manager = SceneManager::new();
    scene_manager.add_scene(Box::new(EditorScene::new(&mut win)), "editor");
    scene_manager.add_scene(Box::new(GameScene::new(&mut win)), "game");
    scene_manager.add_scene(Box::new(MainMenuScene::new(&mut win)), "menu");

    //scene_manager.add_scene(load_editor(&mut win));
    scene_manager.set_current_scene("editor");

    std::thread::spawn(move || {
        ServerManager::new(Server::new()).run()
    });

    let game = TurnBasedStrategy {
        sm: scene_manager,
        gfx: Graphics::new(&mut win),
        inp: inp,
        key: win.create_key_listener(),
        m: win.create_mouse_listener(),
        m_move: win.create_mouse_move_listener(),
        frame: win.create_frame_buffer_listener(),
        win: win,
        client: ClientHandler::new(),
        timer: 0.0,
    };

    GameContainer::new().run(game);
}
