// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 Brass-ape
use chat_core::crypto::KeyPair;

fn main(){
    let keypair = KeyPair::generate().expect("Failed to generate keypair!");

    println!("Public key: {:?}", keypair.public_bytes());
    println!("Keypair generated successfully!");
}