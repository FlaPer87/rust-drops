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

#[crate_id = "drops"];
#[comment = "Drops implementation for rust"];
#[license = "MIT/ASL2"];
#[crate_type = "rlib"];
#[crate_type = "dylib"];

#[feature(phase)];
#[feature(globs)];
#[deny(missing_doc)];

//! Documentation goes here.
extern crate collections;
extern crate green;
extern crate rustuv;
extern crate serialize;
extern crate proton;

#[phase(syntax, link)]
extern crate log;

pub use register::{Register, Args, PluginCallback};
pub use modules::*;

pub mod register;
pub mod modules;
