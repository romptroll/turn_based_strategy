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

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pressed_location_x: f32,
    pressed_location_y: f32,
    moved_amount_x: f32,
    moved_amount_y: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Camera {
        Camera {
            x, y, w, h,
            pressed_location_x: 0.0,
            pressed_location_y: 0.0,
            moved_amount_x: 0.0,
            moved_amount_y: 0.0,
        }
    }

    pub fn zoom(&mut self, x: f32, y: f32, zoom_factor: f32) {
        let w1 = self.w;
        let w2 = self.w * zoom_factor;

        let h1 = self.h;
        let h2 = self.h * zoom_factor;

        self.x = x * (w1 - w2) + self.x;
        self.y = y * (h1 - h2) + self.y;

        self.w *= zoom_factor;
        self.h *= zoom_factor;
    }

    pub fn press(&mut self, x: f32, y: f32) {
        self.pressed_location_x = x;
        self.pressed_location_y = y;
        self.moved_amount_x = 0.0;
        self.moved_amount_y = 0.0;
    }

    pub fn shift(&mut self, x: f32, y: f32) {
        let scl_x = 2.0 / self.w;
        let scl_y = 2.0 / self.h;
        let new_cam_x = (self.pressed_location_x - x) / scl_x + self.x-self.moved_amount_x;
        let new_cam_y = (self.pressed_location_y - y) / scl_y + self.y-self.moved_amount_y;
        
        self.moved_amount_x += new_cam_x-self.x;
        self.moved_amount_y += new_cam_y-self.y;

        self.x = new_cam_x;
        self.y = new_cam_y;
    }
}