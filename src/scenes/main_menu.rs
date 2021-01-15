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

use engine::{core::window::Window, game::GameData, gui::{comps::Button, gui::GUI}, scene::Scene};

pub struct MainMenuScene {
    gui: GUI,
    pub btn_play: Button,
    pub btn_editor: Button,
    pub btn_exit: Button,
}

impl MainMenuScene {
    pub fn new(win: &mut Window) -> MainMenuScene {
        let mut btn_play = Button::new();
        btn_play.x = -0.3;
        btn_play.y = 0.4;
        btn_play.width = 0.6;
        btn_play.height = 0.3;

        let mut btn_editor = Button::new();
        btn_editor.x = -0.3;
        btn_editor.y = 0.0;
        btn_editor.width = 0.6;
        btn_editor.height = 0.3;


        let mut btn_exit = Button::new();
        btn_exit.x = -0.3;
        btn_exit.y = -0.4;
        btn_exit.width = 0.6;
        btn_exit.height = 0.3;



        MainMenuScene {
            gui: GUI::new(win),
            btn_play,
            btn_editor,
            btn_exit,
        }
    }
}

impl Scene for MainMenuScene {
    fn on_render(&mut self, _gd: &mut GameData) {

        self.gui.button(&mut self.btn_play);
        self.gui.button(&mut self.btn_editor);
        self.gui.button(&mut self.btn_exit);

        self.gui.update();
    }
}