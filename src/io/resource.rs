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
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::vec::Vec;

lazy_static!(static ref MAP_HOLDER: HashMap<String, Resource> = HashMap::new(););
lazy_static!(static ref VEC_HOLDER: Vec<Resource> = Vec::new(););
lazy_static!(static ref RESOURCE_HOLDER: Resource = Resource::None;);

pub enum Resource {
    None,
    I64(i64),
    F64(f64),
    Str(String),
    Map(HashMap<String, Resource>),
    Vec(Vec<Resource>),
}

impl Resource {
    pub fn from_file(file_name: &str) -> Resource {
        let mut file = File::open(file_name).expect(&format!("Unable to open {}", file_name));
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect(&format!("Unable to load {}", file_name));
        let yaml_vec = yaml_rust::YamlLoader::load_from_str(&contents).unwrap();
        let yaml = yaml_vec[0].clone();
        
        Resource::rec(yaml)
    }

    pub fn from_string(data: &str) -> Resource {
        let yaml_vec = yaml_rust::YamlLoader::load_from_str(data).unwrap();
        let yaml = yaml_vec[0].clone();
        
        Resource::rec(yaml)
    }

    pub fn to_file(&self, file_name: &str) {
        let mut file = File::create(file_name).expect(&format!("Unable to create {}", file_name));

        let mut out_str = String::new();
        let mut emitter = yaml_rust::YamlEmitter::new(&mut out_str);
        emitter.dump(&Resource::rec_rev(self)).unwrap();

        file.write_all(out_str.as_bytes()).unwrap();
    }

    pub fn to_string(&self) -> String {
        let mut out_str = String::new();
        let mut emitter = yaml_rust::YamlEmitter::new(&mut out_str);
        emitter.dump(&Resource::rec_rev(self)).unwrap();

        out_str
    }

    fn rec(yaml: yaml_rust::Yaml) -> Resource {
        if yaml.as_i64().is_some() {
            return Resource::I64(yaml.as_i64().unwrap());
        } else if yaml.as_f64().is_some() {
            return Resource::F64(yaml.as_f64().unwrap());
        } else if yaml.as_str().is_some() {
            return Resource::Str(yaml.as_str().unwrap().to_string());
        } else if yaml.as_vec().is_some() {
            let v = yaml.as_vec().unwrap();
            
            let mut children = Vec::new();

            for child in v {
                children.push(Resource::rec(child.clone()));
            }

            return Resource::Vec(children);
        } else if yaml.as_hash().is_some() {
            let v = yaml.as_hash().unwrap();
            
            let mut children = HashMap::new();

            for (k,v) in v {
                children.insert(k.as_str().unwrap().to_string(), Resource::rec(v.clone()));
            }

            return Resource::Map(children);
        }
        Resource::None
    }

    fn rec_rev(res: &Resource) -> yaml_rust::Yaml {
        match res {
            Resource::None => {
                return yaml_rust::Yaml::Null
            },
            Resource::F64(num) => {
                return yaml_rust::Yaml::Real(num.to_string())
            },
            Resource::I64(num) => {
                return yaml_rust::Yaml::Integer(*num)
            },
            Resource::Str(s) => {
                return yaml_rust::Yaml::String(s.clone())
            },
            Resource::Vec(v) => {            
                let mut children = Vec::new();

                for child in v {
                    children.push(Resource::rec_rev(child));
                }

                return yaml_rust::Yaml::Array(children)
            },
            Resource::Map(v) => {            
                let mut children = yaml_rust::yaml::Hash::new();

                for (k,v) in v {
                    children.insert(yaml_rust::Yaml::String(k.clone()), Resource::rec_rev(v));
                }

                return yaml_rust::Yaml::Hash(children);
            }
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Resource::Str(s) => {
                Some(s)
            },
            _ => {
                //error_log!("Tried to get resource as String but type was {}!", self.get_type_error());
                None
            }
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Resource::I64(num) => {
                Some(*num)
            },
            _ => {
                //error_log!("Tried to get resource as I64 but type was {}!", self.get_type_error());
                None
            }
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Resource::F64(num) => {
                Some(*num)
            },
            _ => {
                //error_log!("Tried to get resource as F64 but type was {}!", self.get_type_error());
                None
            }
        }
    }

    pub fn as_map(&self) -> Option<&HashMap<String, Resource>> {
        match self {
            Resource::Map(m) => {
                Some(m)
            },
            _ => {
                //error_log!("Tried to get resource as Map but type was {}!", self.get_type_error());
                None
            }
        }
    }

    pub fn as_vec(&self) -> Option<&Vec<Resource>> {
        match self {
            Resource::Vec(v) => {
                Some(v)
            },
            _ => {
                //error_log!("Tried to get resource as Vec but type was {}!", self.get_type_error());
                //&VEC_HOLDER
                None
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&Resource> {
        match self.as_map() {
            Some(map) => {
                map.get(name)
            }
            None => {
                None
            }
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&Resource> {
        match self.as_vec() {
            Some(vec) => {
                vec.get(index)
            }
            None => {
                None
            }
        }
    }

    fn get_type_error(&self) -> &str {
        match self {
            Resource::None => {
                "None"
            },
            Resource::F64(_) => {
                "F64"
            },
            Resource::I64(_) => {
                "I64"
            },
            Resource::Str(_) => {
                "String"
            },
            Resource::Vec(_) => {
                "Vec"
            },
            Resource::Map(_) => {
                "Map"
            }
        }
    }
}