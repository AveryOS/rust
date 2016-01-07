// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use target::{Target, TargetOptions, TargetResult};
use std::default::Default;

pub fn target() -> TargetResult {
    let opts = TargetOptions {
        cpu: "x86-64".to_string(),
        position_independent_executables: true,
        dynamic_linking: true,
        disable_redzone: true,
        no_default_libraries: false,
        eliminate_frame_pointer: true,
        executables: true,
        linker_is_gnu: true,
        pre_link_args: vec!(
            "--target=x86_64-pc-avery".to_string(),
        ),
        linker: "clang".to_string(),
        ar: "x86_64-pc-avery-ar".to_string(),
        archive_format: "gnu".to_string(),
        pre_link_objects_exe: vec!(),
        late_link_args: vec!(),
        post_link_objects: vec!(),
        max_atomic_width: Some(64),
        .. Default::default()
    };

    Ok(Target {
        llvm_target: "x86_64-pc-avery".to_string(),
        data_layout: "e-m:e-i64:64-f80:128-n8:16:32:64-S128".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        arch: "x86_64".to_string(),
        target_os: "avery".to_string(),
        target_env: "".to_string(),
        target_vendor: "unknown".to_string(),
        options: opts,
    })
}
