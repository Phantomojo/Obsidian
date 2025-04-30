# GhostWire

GhostWire is a secure, decentralized platform for threat intelligence sharing and private messaging, designed for cybersecurity professionals and trusted teams.

## Key Features

- Encrypted, anonymous messaging between trusted peers
- Peer-to-peer IOC sharing (malicious IPs, hashes, domains, etc.)
- Decentralized networking with libp2p
- End-to-end encryption with forward secrecy
- Zero metadata storage and privacy-first design
- CLI-first, cross-platform tool with optional Tor/I2P integration

## Project Structure

- `core/` - Core networking, encryption, and storage components
- `cli/` - Command-line interface commands and logic
- `ioc/` - IOC sharing module
- `messaging/` - Messaging module
- `trust/` - Peer trust and reputation system
- `tor_integration/` - Optional Tor/I2P integration

## Getting Started

This project is implemented in Rust for performance, safety, and security.

## License

MIT License
