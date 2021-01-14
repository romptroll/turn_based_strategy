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

use std::{collections::HashMap};

use engine::{core::{info_log, input::{Input, Key}, window::{MouseButtonLeft, Window}}, game::GameData, gui::{comps::Button, gui::GUI}, renderer::{color::{self, Color}, graphics::Graphics, texture::{Texture, TextureRegion}}, scene::Scene};
use crate::{camera::{self, Camera}, io::resource::Resource, map::{Layer, Map, Tile, TileIndex, TileSet}};


pub struct EditorScene {
    inp: Input,
    gui: GUI,
    gfx: Graphics,
    map: Map,
    atlas: std::rc::Rc<Texture>,
    tile_selection: Vec<(Button, TileIndex)>,
    tile_selected: TileIndex,
    camera: Camera,
}

impl EditorScene {
    pub fn new(win: &mut Window) -> EditorScene {
        EditorScene {
            inp: Input::new(win),
            gui: GUI::new(win),
            gfx: Graphics::new(win),
            map: Map::new(),
            atlas: Texture::from_color(1, 1, 0xFFFFFFFF),
            tile_selection: Vec::new(),
            tile_selected: unsafe { TileIndex::new(0) },
            camera: Camera::new(0.0, 0.0, 20.0, 16.0),
        }
    }
}

impl Scene for EditorScene {
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

        for tile in data.get("tiles").unwrap().as_vec().unwrap() {
            let r = tile.get("r").unwrap().as_f64().unwrap() as f32;
            let g = tile.get("g").unwrap().as_f64().unwrap() as f32;
            let b = tile.get("b").unwrap().as_f64().unwrap() as f32;
            let x_off = tile.get("x_off").unwrap().as_f64().unwrap() as f32;
            let y_off = tile.get("y_off").unwrap().as_f64().unwrap() as f32;
            let x_scl = tile.get("x_scl").unwrap().as_f64().unwrap() as f32;
            let y_scl = tile.get("y_scl").unwrap().as_f64().unwrap() as f32;
            let texture = tile.get("texture").unwrap().as_str().unwrap();
            let texture = textures.get(texture).unwrap();
            tiles.add_tile(Tile::new(Color::from((r, g, b, 1.0)), texture.clone(), x_off, y_off, x_scl, y_scl));
        }

        for i in 0..tiles.len() {
            let mut button = Button::new();
            button.x = 64.0 * i as f32;
            button.y = 0.0;
            button.width = 64.0;
            button.height = 64.0;
            self.tile_selection.push((button, unsafe { TileIndex::new(i) } ));
        }

        let map = Map::from_file(&data.get("map").unwrap().as_vec().unwrap()[0].get("path").unwrap().as_str().unwrap(), tiles);

        self.map = map;
        self.atlas = atlas;
    }

    fn on_update(&mut self, gd: &mut GameData) {
        let mouse_x = (self.inp.mouse_x() + 1.0) / self.gfx.scaling().0 + self.camera.x;
        let mouse_y = (self.inp.mouse_y() + 1.0) / self.gfx.scaling().1 + self.camera.y;
        let x = self.inp.mouse_x() / 2.0 + 0.5;
        let y = self.inp.mouse_y() / 2.0 + 0.5;

        if self.inp.mouse(MouseButtonLeft) { 
            //info_log!("{}:{}", mouse_x, mouse_y);

            if (mouse_x as u32) < self.map.layers[0].width && (mouse_y as u32) < self.map.layers[0].height {
                self.map.layers[0].set(mouse_x as u32, mouse_y as u32, self.tile_selected);
            }
            
        }

        if self.inp.mouse_scroll_y() != 0.0 {
            self.camera.zoom(x, y, /*(self.camera.w - self.inp.mouse_scroll_y()) / self.camera.w*/2.0-1.075f32.powf(self.inp.mouse_scroll_y()));
        }

        if self.inp.key_down(Key::R) {
            self.on_start(gd);
        }

        if self.inp.key_down(Key::G) {
            unsafe { self.map.layers[0] = Layer::cellular_automata(100, 100, 100, TileIndex::new(0), TileIndex::new(1)); };
        }

        self.inp.update();
    }

    fn on_render(&mut self, _gd: &mut GameData) {
        //self.gfx.texture(self.map.tile_set().tile(self.map.layers[0].get(0, 0)).texture.clone());
        let scale_x = self.camera.w;
        let scale_y = self.camera.h;
        let off_x = self.camera.x / scale_x;
        let off_y = self.camera.y / scale_y;
        //info_log!("{}", self.camera.x);

        self.gfx.set_color(color::WHITE);
        self.gfx.set_scale(2.0 / scale_x, 2.0 / scale_y);
        self.gfx.set_translation(-1.0 - off_x * 2.0, -1.0 - off_y * 2.0);

        self.map.render(&mut self.gfx);

        self.gfx.update();
        self.gfx.flush();


        self.gui.graphics.set_scale(2.0 / self.gfx.frame_width() as f32, 2.0 / self.gfx.frame_height() as f32);
        self.gui.graphics.set_translation(-1.0, -1.0);

        self.gui.graphics.texture(self.map.tile_set().tile(self.tile_selected).texture.clone());
        self.gui.graphics.fill_rect(0.0, self.gfx.frame_height() as f32 - 64.0, 64.0, 64.0);

        for (b, i) in &mut self.tile_selection {
            self.gui.style.foreground_texture = self.map.tile_set().tile(*i).texture.clone();
            self.gui.style.background_texture = self.map.tile_set().tile(*i).texture.clone();
            self.gui.button(b);

            if b.pressed {
                self.tile_selected = *i;
            }
        }

        self.gui.update();
    }
}