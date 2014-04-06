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

use proton::messenger::Messenger;

fn main() {
    let mut msgr = Messenger::new("test");
    msgr.start();

    msgr.subscribe("amqp://~0.0.0.0");

    loop {
        msgr.recv(1024);
        println!("Received something");

         for msg in msgr {
             println!("Got Message");
             //let data = pn_message_body(message);
             //let mut buf = [0 as libc::c_schar, ..1024u];
             //pn_data_format(data, buf.as_mut_ptr(), &(buf.len() as libc::size_t));
             //let ret = c_str::CString::new (pn_message_get_address(message), true);
             println!("Address {}", msg.get_address());
             println!("Subject {}", msg.get_subject());
             println!("{}", msg.get_body());
             // Calling this, will make the next
             // message get segfault. Needs investigation.
             //pn_message_free(message);
         }

    }
}
