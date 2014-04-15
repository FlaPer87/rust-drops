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

//! Modules loading module

use dl = std::unstable::dynamic_lib;

/// Loads modules
pub fn load(prefix: &Path, name: &str) -> Result<dl::DynamicLibrary, ~str> {
    let x = prefix.join(libname(StrBuf::from_str(name)).into_owned());
    debug!("Full module path: {}", x.display());
    dl::DynamicLibrary::open(Some(&x))
}

#[cfg(target_os="win32")]
fn libname(mut n: StrBuf) -> StrBuf {
    n.push_str(".dll");
    n
}

#[cfg(target_os="macos")]
fn libname(mut n: StrBuf) -> StrBuf {
    n.push_str(".dylib");
    n
}

#[cfg(not(target_os="win32"), not(target_os="macos"))]
fn libname(n: StrBuf) -> StrBuf {
    let mut i = StrBuf::from_str("lib");
    i.push_str(n.as_slice());
    i.push_str(".so");
    i
}
