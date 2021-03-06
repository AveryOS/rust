// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use self::imp::*;

#[cfg(not(any(all(target_os = "ios", target_arch = "arm"), target_os = "avery")))]
#[path = "gcc_s.rs"]
mod imp;
#[cfg(all(target_os = "ios", target_arch = "arm"))]
#[path = "backtrace_fn.rs"]
mod imp;

#[cfg(target_os = "avery")]
mod imp {
	use io;
	use io::prelude::*;
	pub fn write(w: &mut Write) -> io::Result<()> {
	    writeln!(w, "stack backtrace unsupported")?;
	    Ok(())
	}

}