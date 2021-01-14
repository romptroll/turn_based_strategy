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

use engine::ecs::ecs::EntityManager;

use std::vec::Vec;
use std::collections::HashMap;
use std::boxed::Box;
use std::option::Option;

use crate::event::EventHolder;

pub struct SceneManager {
    scenes: HashMap<String, Scene>,
    current_scene: Option<String>,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            scenes: HashMap::new(),
            current_scene: None,
        }
    }

    pub fn update(&mut self, delta_time: f32, fps: u32) -> bool {
        match &self.current_scene {
            Some(name) => self.scenes.get_mut(name).unwrap().update(delta_time, fps),
            None => {
                true
            }
        }
    }

    pub fn event(&mut self, event: EventHolder) -> bool {
        match &self.current_scene {
            Some(name) => self.scenes.get_mut(name).unwrap().on_event(event),
            None => true
        }
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.insert(scene.name.clone(), scene);
    }

    pub fn set_current_scene(&mut self, name: &str) {
        if name == "" {
            self.current_scene = None;
        } else if !self.scenes.contains_key(name)  {
            error_log!("Tried to set current scene to: \"{}\" but scene didn't exist!", name);
        } else {
            self.current_scene = Some(name.to_string());
        }
    }

    pub fn get_current_scene_name(&self) -> &str {
        match &self.current_scene {
            Some(name) => &name,
            None => ""
        }
    }

    pub fn get_current_scene(&self) -> Option<&Scene> {
        match &self.current_scene {
            Some(name) => Some(&self.scenes[name]),
            None => None,
        }
    }

    pub fn get_current_scene_mut(&mut self) -> Option<&mut Scene> {
        match &self.current_scene {
            Some(name) => self.scenes.get_mut(name),
            None => None,
        }
    }
}

pub struct Scene {
    pub name: String,
    pub em: EntityManager,
    pub events: Vec<EventHolder>,
    systems: Vec<Box<dyn crate::systems::system::System>>,
}

impl Scene {

    pub fn new(name: &str) -> Scene {
        Scene {
            em: EntityManager::new(),
            name: name.to_string(),
            events: Vec::new(),
            systems: Vec::new(),
        }
    }

    pub fn update(&mut self, delta_time: f32, fps: u32) -> bool {
        let mut healthy = true;
        for system in &mut self.systems {
            let (mut events, h) = system.update(&mut self.em, delta_time, fps);
            healthy = h;
            self.events.append(&mut events);
        }
        healthy
    }

    pub fn on_event(&mut self, event: EventHolder) -> bool {
        let mut healthy = true;
        for system in &mut self.systems {
            let (mut events, h) = system.event(&mut self.em, event.clone());
            healthy = h;
            self.events.append(&mut events);
        }
        healthy
    }

    pub fn broadcast_event(&mut self, event: EventHolder) {
        self.events.push(event);
    }

    pub fn add_system(&mut self, system: Box<dyn crate::systems::system::System>) {
        self.systems.push(system);
    }
}