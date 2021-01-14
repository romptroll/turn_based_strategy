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
}

impl Camera {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Camera {
        Camera {
            x, y, w, h,
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
}