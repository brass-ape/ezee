// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 Brass-ape
use chat_core::crypto::KeyPair;

fn main(){
    let keypair = KeyPair::generate().expect("failed");
    let bytes = keypair.to_bytes();
    println!("{:?}", bytes.secret);
    println!("{:?}", keypair.public_bytes());
    drop(keypair);
}