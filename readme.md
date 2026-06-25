<div align="center">

**End-to-end encrypted, zero-trust, privacy-first messaging in Rust.**

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Status: In Development](https://img.shields.io/badge/Status-In%20Development-yellow.svg)]()
[![Encryption: ChaCha20--Poly1305](https://img.shields.io/badge/Encryption-ChaCha20--Poly1305-green.svg)]()

</div>

---

> *"The server should be structurally incapable of reading your messages — not just policy-level 'we promise not to look'."*

---

## What is ezee?

ezee is a **terminal-first, end-to-end encrypted chat client** written in Rust, built around a single principle: **zero trust**. The server is a dumb relay. It stores encrypted blobs it cannot read, serves public keys it cannot misuse, and routes messages between parties it cannot surveil.

No telemetry. No metadata harvesting. No corporate backend. Self-host in ten minutes.

The codebase is deliberately modular — the core cryptographic and networking logic lives entirely in `chat-core`, a library crate with no knowledge of how it is displayed. The terminal UI (`chat-cli`) is a thin shell today. A GUI (`chat-gui`) can be bolted on tomorrow without touching a line of cryptographic code.

---

## Ideology

### Zero Trust

Trust is a vulnerability. ezee is designed so that a fully compromised server — whether by an attacker, a subpoena, or a malicious operator — reveals nothing about message content. The server never sees plaintext. It never holds private keys. It cannot be compelled to hand over what it does not have.

### Cryptography as Architecture

Security is not a feature you add at the end. Every structural decision in ezee — module visibility, error handling, memory management — is made with the threat model in mind. Private keys are inaccessible outside their module by language-level enforcement. Session keys are zeroed from memory the moment they are no longer needed. Error messages deliberately discard detail that could enable side-channel attacks.

### Forward Secrecy

Stealing today's keys does not decrypt yesterday's messages. ezee uses the **Double Ratchet Algorithm** — the same protocol underpinning Signal — ensuring that each message is encrypted with an ephemeral key that is deleted after use. Compromise of one session key is contained; it does not cascade.

### Defence in Depth

No single mechanism is sufficient. ezee layers protections:

- **Cryptographic** — X25519 key exchange, ChaCha20-Poly1305 AEAD, Argon2id key derivation
- **Process** — `seccomp` syscall filtering, Linux namespace isolation
- **Memory** — `mlock` to prevent key material reaching swap, `zeroize` to wipe it on drop
- **Authentication** — Three-layer auth stack: identity (keypair), protection (passphrase), access (PIN challenge)

### Partial Knowledge PIN Challenge

Borrowed from UK retail banking, the PIN challenge never asks for the full PIN in a single session. A keylogger capturing input across a hundred sessions still cannot reconstruct the full PIN, because the positions requested rotate randomly every time. It is not a cryptographic primitive — it is a human-factors defence against a realistic attack vector.

### Open Source, Always

ezee is and will remain GPL-3.0. Security software that cannot be audited cannot be trusted. The licence ensures that any derivative work must also be open — no proprietary forks, no closed backends, no hidden modifications.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        chat-core (lib)                          │
│                                                                 │
│  ┌──────────┐  ┌──────────┐  ┌────────┐  ┌────────────────┐   │
│  │  crypto  │  │ identity │  │  auth  │  │   messaging    │   │
│  │          │  │          │  │        │  │                │   │
│  │ X25519   │  │ KeyPair  │  │Argon2  │  │ Composer       │   │
│  │ ChaCha20 │  │ Profile  │  │PINVault│  │ Receiver       │   │
│  │ Ratchet  │  │          │  │Session │  │ DoubleRatchet  │   │
│  └──────────┘  └──────────┘  └────────┘  └────────────────┘   │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                    server (HTTP client)                   │  │
│  │         reqwest · serde · configurable endpoint          │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────────┬────────────────────────────────────┘
                             │  same API
              ┌──────────────┴──────────────┐
              │                             │
   ┌──────────▼──────────┐     ┌────────────▼────────────┐
   │      chat-cli        │     │        chat-gui          │
   │   (terminal shell)   │     │    (future · egui?)      │
   │   thin · no logic    │     │    thin · no logic       │
   └──────────────────────┘     └─────────────────────────┘
```

```
┌────────────────────────────────────────────────────────────────┐
│                     Authentication Stack                        │
│                                                                 │
│   Layer 1 ──▶  Identity       X25519 keypair proves who you are│
│   Layer 2 ──▶  Protection     Passphrase encrypts key at rest  │
│   Layer 3 ──▶  Access         Partial PIN gates message decrypt│
└────────────────────────────────────────────────────────────────┘
```

---

## Cryptographic Primitives

| Purpose              | Primitive                  | Crate                  |
|----------------------|----------------------------|------------------------|
| Key exchange         | X25519 Diffie-Hellman      | `x25519-dalek`         |
| Message encryption   | ChaCha20-Poly1305 AEAD     | `chacha20poly1305`     |
| Key signing          | Ed25519                    | `ed25519-dalek`        |
| Key derivation       | Argon2id                   | `argon2`               |
| Session ratchet      | Double Ratchet Algorithm   | `double-ratchet` (WIP) |
| Secure random        | OS entropy (OsRng)         | `rand`                 |
| Memory zeroing       | Zeroize on drop            | `zeroize`              |
| Constant-time cmp    | ConstantTimeEq             | `subtle`               |

---

## Project Status

> 🚧 **Active Development** — not yet suitable for production use.

| Component                        | Status         | Notes                                      |
|----------------------------------|----------------|--------------------------------------------|
| `crypto/error.rs`                | ✅ Complete     | Custom error types, thiserror              |
| `crypto/keys.rs`                 | ✅ Complete     | X25519 keypair generation                  |
| `crypto/encrypt.rs`              | ✅ Complete     | ChaCha20-Poly1305, tests passing           |
| `identity/keypair.rs`            | 🔨 In Progress  | Save/load keypair to disk                  |
| `auth/passphrase.rs`             | ⬜ Planned      | Argon2id key derivation                    |
| `auth/pin_vault.rs`              | ⬜ Planned      | Partial PIN challenge system               |
| `auth/session.rs`                | ⬜ Planned      | Session state management                   |
| `server/client.rs`               | ⬜ Planned      | HTTP client, configurable endpoint         |
| `messaging/composer.rs`          | ⬜ Planned      | Outgoing message pipeline                  |
| `messaging/receiver.rs`          | ⬜ Planned      | Incoming message pipeline                  |
| `messaging/ratchet.rs`           | ⬜ Planned      | Double Ratchet forward secrecy             |
| `chat-cli`                       | ⬜ Planned      | Terminal interface                         |
| `chat-gui`                       | ⬜ Future       | Graphical interface (egui / Tauri)         |
| Server (FastAPI / Axum)          | ⬜ Planned      | Zero-knowledge relay                       |
| Sealed sender                    | ⬜ Future       | Hide sender identity from server           |
| `seccomp` sandboxing             | ⬜ Future       | Syscall filtering                          |

---

## Setup

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain, 1.75+)
- Git

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Clone & Build

```bash
git clone https://github.com/yourusername/ezee.git
cd ezee

# Build everything
cargo build

# Run tests
cargo test

# Run the CLI (once implemented)
cargo run -p chat-cli
```

### Configuration

> Configuration file format is not yet finalised. The planned location is `~/.config/ezee/config.toml`.

The server endpoint will be user-configurable — point it at your own instance or a trusted community server:

```toml
# ~/.config/ezee/config.toml  (planned)
server_url = "https://chat.yourdomain.com"
username   = "your_username"
```

### Self-Hosting the Server

> Server implementation is planned. Instructions will be added here when the server crate is complete.

The server is intentionally minimal — it requires no special infrastructure. A small VPS running the server binary behind Caddy (for automatic TLS) is sufficient.

---

## Threat Model

ezee protects against:

- ✅ Server reading message content (zero-trust E2EE)
- ✅ Stolen key files (Argon2id-encrypted at rest)
- ✅ Session hijacking (PIN challenge as second factor)
- ✅ Past message exposure if keys are stolen (Double Ratchet forward secrecy)
- ✅ Brute force (Argon2id slow hashing + rate limiting)
- ✅ Memory forensics (zeroize + mlock)
- ✅ Keyloggers across sessions (partial PIN rotation)
- ⚠️ Message metadata / traffic analysis (partial — sealed sender planned)
- ❌ Kernel rootkits or hardware keyloggers (out of scope)
- ❌ Screen recorders (out of scope)

---

## Contributing

ezee is a learning project built in public. Issues, questions, and pull requests are welcome. If you find a security issue, please open a private issue rather than a public one.

Code style: `cargo fmt` before committing. `cargo clippy` should pass with no warnings.

---

## Licence

Copyright (C) 2026 Brass-ape

This program is free software: you can redistribute it and/or modify it under the terms of the **GNU General Public License** as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but **without any warranty**; without even the implied warranty of merchantability or fitness for a particular purpose. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

---

<div align="center">
<sub>Built with Rust · Secured with mathematics · Trusting no one</sub>
</div>