/* SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only  */
/* Copyright (c) 2021 - 2024 Gavin Henry <ghenry@sentrypeer.org> */
/*
   _____            _              _____
  / ____|          | |            |  __ \
 | (___   ___ _ __ | |_ _ __ _   _| |__) |__  ___ _ __
  \___ \ / _ \ '_ \| __| '__| | | |  ___/ _ \/ _ \ '__|
  ____) |  __/ | | | |_| |  | |_| | |  |  __/  __/ |
 |_____/ \___|_| |_|\__|_|   \__, |_|   \___|\___|_|
                              __/ |
                             |___/
*/

fn main() {
    // Tell cargo to tell rustc to link the sentrypeer
    // shared library.
    println!("cargo:rustc-link-search=../.libs");
    println!("cargo:rustc-link-lib=sentrypeer");
    println!("cargo:rustc-link-lib=opendht-c");
    println!("cargo:rustc-link-lib=jansson");
    println!("cargo:rustc-link-lib=uuid");
}
