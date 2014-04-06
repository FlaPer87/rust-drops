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

extern crate drops_console;
extern crate drops;
extern crate serialize;

use serialize::json;
use drops::register::Register;

#[test]
fn test_register_console() {
    let mut reg = Register::new("/home/flaper87/workspace/personal/rust-drops/build");
    reg.load("drops_console");
    reg.call("info", Some(json::Decoder::new(json::from_str("\"My String\"").unwrap())));
    reg.call("info", None);
}
