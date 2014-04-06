// Licensed under the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License. You may obtain
// a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.

//! Register Module

use dl = std::unstable::dynamic_lib;
use collections::hashmap::HashMap;
use serialize::{json, Decodable};

use modules;

/// Dynamic args
pub type Args = Option<json::Decoder>;
pub type PluginCallback = extern "Rust" fn(Args);

/// Registers struct
pub struct Register<'r> {
    /// Path where modules are located
    prefix: Path,

    /// (worker, library) map
    libs: HashMap<&'r str, dl::DynamicLibrary>,

    /// (method, callback) map
    methods: HashMap<&'r str, PluginCallback>,
}

/// Register trait
impl<'r> Register<'r> {

    /// Static constructor
    pub fn new(path: &'r str) -> Register {
        Register {
            prefix: Path::new(path),
            libs: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    /// Lookup a method
    pub fn lookup(&self, method: &'r str) -> Option<PluginCallback> {
        debug!("Looking for method={}, number of registered methods={}",
               method, self.methods.len())
        match self.methods.find(&method) {
            Some(callback) => {
                Some(*callback)
            }
            _ => {fail!("erm, wrong!")}
        }
    }

    /// Find and call method
    pub fn call(&self, method: &'r str, args: Args) {
        self.lookup(method).unwrap()(args)
    }

    /// Registers a method
    pub fn load(&mut self, worker: &'r str) {
        match modules::load(&self.prefix, worker.to_owned()) {
            Ok(lib) => {
                debug!("Library {} found", worker);
                let register: Result<fn(&mut Register), ~str> = unsafe {
                    lib.symbol("register_worker")
                };

                match register {
                    Ok(reg) => {reg(self);}
                    _ => { info!("Worker {} has no register", worker)}
                }

                self.libs.insert(worker, lib);
            },
            _ => {info!("Worker module {} could not be found!", worker)}
        }
    }

    /// Register new methods
    #[no_mangle]
    pub fn register(&mut self, method: &'r str, f: PluginCallback) {
        self.methods.insert(method, f);
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_register() {
        let mut reg = Register::new("/home/flaper87/workspace/personal/rust-drops/build");
        reg.load("drops_console");
    }

}
