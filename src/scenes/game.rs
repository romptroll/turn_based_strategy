/*
 *   Copyright (c) 2021 Ludwig Bogsveen
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

use std::{collections::HashMap};

use engine::{core::{info_log, input::{Input, Key}, window::{MouseButtonMiddle, Window}}, game::GameData, gui::{comps::Button, gui::GUI}, renderer::{color::{self, Color}, graphics::Graphics, texture::{Texture, TextureRegion}}, scene::Scene};
use crate::{camera::{Camera}, io::resource::Resource, map::{Map, Tile, TileIndex, TileSet}};
 

pub struct GameScene {
    inp: Input,
    gui: GUI,
    gfx: Graphics,
    map: Map,
    atlas: std::rc::Rc<Texture>,
    camera: Camera,
}

impl GameScene {
    pub fn new(win: &mut Window) -> GameScene {
        GameScene {
            inp: Input::new(win),
            gui: GUI::new(win),
            gfx: Graphics::new(win),
            map: Map::new(),
            atlas: Texture::from_color(1, 1, 0xFFFFFFFF),
            camera: Camera::new(0.0, 0.0, 20.0, 16.0),
        }
    }
}

impl Scene for GameScene {
    fn on_start(&mut self, _gd: &mut GameData) {
        let data = Resource::from_file("res/data_jt.yaml");

        let atlas_path = data.get("atlas").unwrap().get_index(0).unwrap().get("path").unwrap().as_str().unwrap();
        let atlas = Texture::from_file(atlas_path);

        let mut textures = HashMap::new();

        for texture in data.get("textures").unwrap().as_vec().unwrap() {
            let texture_name = texture.get("name").unwrap().as_str().unwrap();
            let bounds = texture.get("bounds").unwrap().as_vec().unwrap();
            
            let w = bounds[2].as_i64().unwrap() as u32;
            let h = bounds[3].as_i64().unwrap() as u32;
            let x = bounds[0].as_i64().unwrap() as u32;
            let y = atlas.height() - bounds[1].as_i64().unwrap() as u32 - h;

            let texture = TextureRegion::new(x, y, w, h, &atlas);
            
            textures.insert(texture_name, texture);
        }


        
        let mut tiles = TileSet::new();

        for tile_d in data.get("tiles").unwrap().as_vec().unwrap() {
            let r = match tile_d.get("r") {Some(x) => x.as_f64().unwrap() as f32, None => 1.0};
            let g = match tile_d.get("g") {Some(x) => x.as_f64().unwrap() as f32, None => 1.0};
            let b = match tile_d.get("b") {Some(x) => x.as_f64().unwrap() as f32, None => 1.0};
            let texture = tile_d.get("texture").unwrap().as_str().unwrap();
            let texture = textures.get(texture).unwrap();
            let mut tile = Tile::new(Color::from((r, g, b, 1.0)), texture.clone());
            tile.x_off = match tile_d.get("x_off") {Some(x) => x.as_f64().unwrap() as f32, None => 0.0};
            tile.y_off = match tile_d.get("y_off") {Some(x) => x.as_f64().unwrap() as f32, None => 0.0};
            tile.x_scl = match tile_d.get("x_scl") {Some(x) => x.as_f64().unwrap() as f32, None => 1.0};
            tile.y_scl = match tile_d.get("y_scl") {Some(x) => x.as_f64().unwrap() as f32, None => 1.0};
            tiles.add_tile(tile);
        }

        let map = Map::from_file(&data.get("map").unwrap().as_vec().unwrap()[0].get("path").unwrap().as_str().unwrap(), tiles);

        self.map = map;
        self.atlas = atlas;
    }

    fn on_update(&mut self, gd: &mut GameData) {
        let x = self.inp.mouse_x() / 2.0 + 0.5;
        let y = self.inp.mouse_y() / 2.0 + 0.5;

        if self.inp.mouse_down(MouseButtonMiddle) {
            self.camera.press(self.inp.mouse_x(), self.inp.mouse_y())
        }

        if self.inp.mouse(MouseButtonMiddle) {
            self.camera.shift(self.inp.mouse_x(), self.inp.mouse_y())
        }

        if self.inp.mouse_scroll_y() != 0.0 && !self.inp.mouse(MouseButtonMiddle) {
            self.camera.zoom(x, y, 2.0-1.075f32.powf(self.inp.mouse_scroll_y()));
        }

        if self.inp.key_down(Key::R) {
            self.on_start(gd);
        }

        self.inp.update();
    }

    fn on_render(&mut self, _gd: &mut GameData) {
        let scale_x = self.camera.w;
        let scale_y = self.camera.h;
        let off_x = self.camera.x / scale_x;
        let off_y = self.camera.y / scale_y;

        self.gfx.set_color(color::WHITE);
        self.gfx.set_scale(2.0 / scale_x, 2.0 / scale_y);
        self.gfx.set_translation(-1.0 - off_x * 2.0, -1.0 - off_y * 2.0);

        self.map.render(&mut self.gfx);

        self.gfx.update();
        self.gfx.flush();


        self.gui.graphics.set_scale(2.0 / self.gfx.frame_width() as f32, 2.0 / self.gfx.frame_height() as f32);
        self.gui.graphics.set_translation(-1.0, -1.0);

        self.gui.update();
    }
}