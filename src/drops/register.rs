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
use rustuv;
use std::rt;
use std::task::TaskOpts;
use serialize::{json, Decodable};
use green::{SchedPool, PoolConfig};
use proton::messenger::Messenger;

use modules;

/// Dynamic args
pub type Args = Option<json::Decoder>;
pub type PluginCallback = extern "Rust" fn(Args);

/// Listener struct
struct Listener<'l> {
    module: &'l str,
    method: &'l str,
    cb: PluginCallback,
}


/// Listener Impl
impl<'l> Listener<'l> {

    pub fn new(module: &'l str, method: &'l str, cb: PluginCallback) -> Listener<'l> {
        Listener {
            module: module,
            method: method,
            cb: cb
        }
    }
}


/// Registers struct
pub struct Register<'r> {
    /// Path where modules are located
    prefix: Path,

    scheduler: SchedPool,
}
/// (method, callback) map
    ///methods: HashMap<&'r str, Listener<'r>>,

/// Register trait
impl<'r> Register<'r> {

    /// Static constructor
    pub fn new(path: &'r str) -> Register {
        let config = PoolConfig {threads: rt::default_sched_threads(),
                                 event_loop_factory: rustuv::event_loop};
        let sched = SchedPool::new(config);

        Register {
            scheduler: sched,
            prefix: Path::new(path),
            //methods: HashMap::new(),
        }
    }

    /// Lookup a method
    //pub fn lookup(&self, method: &'r str) -> Option<Listener<'r>> {
    //    debug!("Looking for method={}, number of registered methods={}",
    //           method, self.methods.len())
    //    match self.methods.find(&method.to_owned()) {
    //        Some(callback) => {
    //            Some(*callback)
    //        }
    //        _ => {fail!("erm, wrong!")}
    //    }
    //}

    /// Find and call method
    pub fn call(&self, method: &'r str, args: Args) {
        //self.lookup(method).unwrap()(args)
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
            },
            _ => {info!("Worker module {} could not be found!", worker)}
        }
    }

    /// Register new methods
    #[no_mangle]
    pub fn register(&mut self, module: &'r str, method: &'r str, f: PluginCallback) {
        let l = Listener::new(module, method, f);
        //self.methods.insert(method.to_owned(), l);
        self.scheduler.spawn(TaskOpts::new(), proc() {
            let mut msgr = Messenger::new("test");
            msgr.start();
            msgr.subscribe("amqp://~0.0.0.0");

            loop {
                msgr.recv(1024);
                debug!("Received something");

                for msg in msgr {
                    debug!("Got Message");
                    //let data = pn_message_body(message);
                    //let mut buf = [0 as libc::c_schar, ..1024u];
                    //pn_data_format(data, buf.as_mut_ptr(), &(buf.len() as libc::size_t));
                    //let ret = c_str::CString::new (pn_message_get_address(message), true);
                    debug!("Address {}", msg.get_address());
                    debug!("Subject {}", msg.get_subject());
                    debug!("{}", msg.get_body());
                    // Calling this, will make the next
                    // message get segfault. Needs investigation.
                    //pn_message_free(message);
                }
            }
        })
    }
}

impl<'r> Drop for Register<'r> {
    fn drop(&mut self) {
        println!("{}", self.prefix.display());

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
