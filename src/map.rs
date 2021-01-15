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

use std::collections::HashMap;
use engine::{core::info_log, renderer::{color::{self, Color, WHITE}, graphics::Graphics, texture::{Texture, TextureRegion}}};

use crate::io::resource::Resource;

#[derive(Clone, Copy, PartialEq)]
pub struct TileIndex(usize);

impl TileIndex {
    pub unsafe fn new(index: usize) -> TileIndex {
        TileIndex(index)
    }
}

#[derive(Clone)]
pub struct Tile {
    pub color: Color,
    pub texture: TextureRegion,
    pub x_off: f32,
    pub y_off: f32,
    pub x_scl: f32,
    pub y_scl: f32,
}

impl Tile {
    pub fn new(color: Color, texture: TextureRegion) -> Tile {
        Tile {
            color,
            texture,
            x_off: 0.0,
            y_off: 0.0,
            x_scl: 0.0,
            y_scl: 0.0,
        }
    }

    pub fn new_invalid() -> Tile {
        Tile {
            color: WHITE,
            texture: TextureRegion::new_invalid(),
            x_off: 0.0,
            y_off: 0.0,
            x_scl: 0.0,
            y_scl: 0.0,
        }
    }

    pub fn from_color(color: Color, id: TileIndex) -> Tile {
        Tile {
            color: color,
            texture: TextureRegion::new_invalid(),
            x_off: 0.0,
            y_off: 0.0,
            x_scl: 0.0,
            y_scl: 0.0,
        }
    }
}

pub struct TileSet {
    tiles: Vec<Tile>,
}

impl TileSet {
    pub fn new() -> TileSet {
        TileSet { tiles: vec![ Tile::new_invalid() ], }
    }

    pub fn add_tile(&mut self, tile: Tile) -> TileIndex {
        self.tiles.push(tile);
        TileIndex(self.tiles.len()-1)
    }

    pub fn tile(&self, index: TileIndex) -> &Tile {
        &self.tiles[index.0]
    }

    pub fn tile_mut(&mut self, index: TileIndex) -> &mut Tile {
        &mut self.tiles[index.0]
    }

    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
}

#[derive(Clone)]
pub struct Layer {
    tiles: Vec<TileIndex>,
    pub width: u32,
    pub height: u32,
}

impl Layer {
    pub fn new(width: u32, height: u32) -> Layer {
        let mut tiles = Vec::new();
        tiles.resize((width*height) as usize, TileIndex(0));
        Layer {
            tiles,
            width,
            height,
        }
    }

    pub fn from_file(path: &str, tile_set: &TileSet) -> Layer {
        let layer_data = Resource::from_file(path);


        let width = layer_data.get("width").unwrap().as_i64().unwrap() as u32;
        let height = layer_data.get("height").unwrap().as_i64().unwrap() as u32;
        let index = layer_data.get("index").unwrap().as_i64().unwrap() as usize;
        let map_data = layer_data.get("data").unwrap().as_vec().unwrap();
        
        let mut layer = Layer::new(width, height);
        
        let mut tile_index = 0;
        for tile in map_data {
            let x = tile_index % width;
            let y = height - 1 - (tile_index - x) / width;
            layer.set(x, y, TileIndex(tile.as_i64().unwrap() as usize % tile_set.len()));
            tile_index += 1;
        }

        layer
    }

    pub fn cellular_automata(width: u32, height: u32, iterations: u32, tile_1: TileIndex, tile_2: TileIndex) -> Layer {
        let mut map = Layer::new(width, height);
        let mut swap_map = Layer::new(width, height);
    
        for i in 0..map.width {
            for j in 0..map.height {
                if rand::random() {
                    map.set(i, j, tile_1);
                }
                else {
                    map.set(i, j, tile_2);
                }
            }
        }
    
        for _ in 0..iterations {
            for x in 0..map.width {
                for y in 0..map.height {
                    let mut n = 0;
                    for i in 0..3 {
                        for j in 0..3 {
                            let mut real_x = (x + i) as i32 - 1;
                            let mut real_y = (y + j) as i32 - 1;
    
                            if real_x < 0 {
                                real_x = map.width as i32 - 1;
                            }
                            if real_y < 0 {
                                real_y = map.height as i32 - 1;
                            }
    
                            if real_x >= map.width as i32 {
                                real_x = 0;
                            }
                            if real_y >= map.height as i32 {
                                real_y = 0;
                            }
    
                            let real_x = real_x as u32;
                            let real_y = real_y as u32;
    
                            if map.get(real_x, real_y) == tile_1 {
                            n += 1;
                        }         
                    }
                    }
                    if map.get(x, y) == tile_1 {
                        n -= 1;
                    }
    
                    if n < 4 {
                        swap_map.set(x, y, tile_2);
                    } else if n > 4 {
                        swap_map.set(x, y, tile_1);
                    }
                    else {
                        swap_map.set(x, y, map.get(x, y));
                    }
                }
            }
            for x in 0..map.width {
                for y in 0..map.height {
                    map.set(x, y, swap_map.get(x, y));
                }
            }
        }
    
        map
    }

    pub fn render(&self, gfx: &mut Graphics, tiles: &TileSet) {
        for y in (0..self.height).rev() { 
            for x in 0..self.width {

                let index = self.get(x, y);
                let tile = tiles.tile(index);

                if index == TileIndex(0) {
                    continue;
                }

                let x_off = tile.x_off / tile.texture.width as f32;
                let y_off = tile.y_off / tile.texture.height as f32;
                
                gfx.texture(tile.texture.clone());
                gfx.set_color(tile.color);
                gfx.fill_rect(x as f32 + x_off, y as f32 + y_off, tile.x_scl, tile.y_scl);
            }
        }
    }

    pub fn get(&self, x: u32, y: u32) -> TileIndex {
        /*if x >= self.width || y >= self.height {
            error_log!("Tried to get tile outside map range! Map size: {}, {} Tile pos: {}, {}", self.width, self.height, x, y);
            return 0;
        }*/
        self.tiles[(x + y * self.width) as usize]
    }
    
    pub fn set(&mut self, x: u32, y: u32, tile_id: TileIndex) {
        /*if x >= self.width || y >= self.height {
            error_log!("Tried to set tile outside map range! Map size: {}, {} Tile pos: {}, {}", self.width, self.height, x, y);
            return;
        }*/
        self.tiles[(x + y * self.width) as usize] = tile_id;
    }
}
pub struct Map {
    pub layers: Vec<Layer>,
    tile_set: TileSet,
}

impl Map {
    pub fn new() -> Map {
        Map {
            layers: Vec::new(),
            tile_set: TileSet::new(),
        }
    }

    pub fn from_file(path: &str, tile_set: TileSet) -> Map {
        let res = Resource::from_file(path);

        let mut map = Map::new();

        for layer_data in res.get("layers").unwrap().as_vec().unwrap() {
            let width = layer_data.get("width").unwrap().as_i64().unwrap() as u32;
            let height = layer_data.get("height").unwrap().as_i64().unwrap() as u32;
            let index = layer_data.get("index").unwrap().as_i64().unwrap() as usize;
            let map_data = layer_data.get("data").unwrap().as_vec().unwrap();
            
            let mut layer = Layer::new(width, height);
            
            let mut tile_index = 0;
            for tile in map_data {
                let x = tile_index % width;
                let y = height - 1 - (tile_index - x) / width;
                layer.set(x, y, TileIndex(tile.as_i64().unwrap() as usize % tile_set.len()));
                tile_index += 1;
            }
            map.layers.resize_with(index + 1, || Layer::new(0, 0));
            map.layers[index] = layer;
        }

        map.tile_set = tile_set;

        map
    }

    pub fn to_file(&self, path: &str) {
        let mut layers = Vec::new();

        for layer in &self.layers {
            let mut map_data = HashMap::new();
            map_data.insert("width".to_string(), Resource::I64(layer.width as i64));
            map_data.insert("height".to_string(), Resource::I64(layer.height as i64));

            let mut tiles = Vec::new(); 

            for y in 0..layer.height {
                for x in 0..layer.width {
                    tiles.push(Resource::I64(layer.get(x, layer.height - 1 - y).0 as i64));
                }
            }

            map_data.insert("data".to_string(), Resource::Vec(tiles));
            layers.push(Resource::Map(map_data));
        }

        Resource::Vec(layers).to_file(path);
    }

    pub fn render(&self, gfx: &mut Graphics) {
        for layer in &self.layers {
            layer.render(gfx, &self.tile_set);
        }
    }

    pub fn tile_set(&self) -> &TileSet {
        &self.tile_set
    }
}