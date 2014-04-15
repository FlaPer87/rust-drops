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

#[feature(phase)];
#[feature(globs)];
#[deny(missing_doc)];

//! Documentation goes here.
extern crate collections;
extern crate proton;
extern crate drops;

#[phase(syntax, link)]
extern crate log;

use drops::register::Register;
use proton::messenger::Messenger;

fn main() {
    //let mut reg = Register::new("/home/flaper87/workspace/personal/rust-drops/build");
    //reg.load("drops_console");

    let mut msgr = Messenger::new("test");
    msgr.start();
    msgr.subscribe("amqp://~0.0.0.0/test.test");

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

    //reg.shutdown();
}
