# Encrypted Vault (Rust)

A cross-platform encrypted secrets vault built in Rust. The goal of this project is to design and implement a secure, portable password manager that stores all sensitive data in an encrypted file that can be carried on removable storage (e.g. USB drives), cloud integration, or local storage.

This project focuses on secure software design principles including strong encryption, safe memory handling, and clear separation between data storage and application logic.

# Phase 1 - Unencrypted MVP

The current implementation provides:

* Vault data model (entries with username, password, URL, notes)
* Basic CRUD operations (add, list, delete entries)
* JSON-based local storage (unencrypted for development only)
* Simple command-line execution flow

# Planned Features

## Phase 2 — Encryption Layer

* Replace plaintext storage with encrypted vault file (`vault.enc`)
* Implement:
  * Argon2id for key derivation from master password
  * ChaCha20-Poly1305 for authenticated encryption
* Introduce secure salt and nonce handling
* Ensure tamper detection via authentication tags

## Phase 3 — Secure CLI Interface

* Interactive command-line interface
* Vault unlock / lock flow
* Entry management commands (add, view, delete)

## Phase 4 — Security Enhancements

* Automatic vault locking after inactivity
* Secure memory handling (zeroization of sensitive data)
* Clipboard timeout clearing (optional)
* Atomic file writes to prevent corruption

## Phase 5 — USB / Portable Usage

* Support opening vault from removable storage (USB)
* Portable workflow:
  * Insert USB → run program → unlock vault → edit → save → eject
* Ensure no dependency on device-specific state

---

# Future Ideas

* Optional keyfile-based unlocking (2-factor vault access)
* Encrypted backup snapshots
* GUI interface (Tauri or similar cross-platform framework)
* Vault sync between multiple encrypted storage locations
* Hardware-backed authentication (advanced)

---

# Security Model (High-Level)

The system assumes:

* The vault file (`vault.enc`) may be publicly exposed
* Security relies entirely on:

  * Master password strength
  * Cryptographic key derivation (Argon2id)
  * Authenticated encryption (ChaCha20-Poly1305)

Threats considered:

* Stolen storage device (USB / disk)
* Offline brute-force attempts
* Vault file tampering

Out of scope:

* Malware on compromised systems
* Keyloggers or runtime memory extraction attacks
* Physical hardware compromise

---

# Tech Stack

* Rust (core application)
* serde / serde_json (serialization)
* argon2 (key derivation)
* chacha20poly1305 (encryption)
* rand (secure randomness)
* zeroize (secure memory cleanup)