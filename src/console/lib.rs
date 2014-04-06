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

#[crate_id = "drops_console"];
#[comment = "Console worker for drops"];
#[license = "MIT/ASL2"];
#[crate_type = "staticlib"];
#[crate_type = "dylib"];
#[feature(phase)];

#[phase(syntax, link)]
extern crate log;
extern crate drops;
extern crate serialize;

use drops::register;
use serialize::{json, Decodable};

#[no_mangle]
pub fn register_worker(register: &mut register::Register) {
    register.register("info", info);
}

pub fn info(arg: register::Args) {
    let msg: ~str = match arg {
        Some(decoder) => {
            let mut d = decoder;
            Decodable::decode(&mut d)
        }
        None => {~"No String"}
    };

    info!("{}", msg);
}
