# wol-relay

[![Crates.io](https://img.shields.io/crates/v/wol-relay)](https://crates.io/crates/wol-relay)
[![Crates.io](https://img.shields.io/crates/d/wol-relay)](https://crates.io/crates/wol-relay)
[![License](https://img.shields.io/crates/l/wol-relay)](https://github.com/Finomnis/wol-relay/blob/main/LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Finomnis/wol-relay/ci.yml?branch=main)](https://github.com/Finomnis/wol-relay/actions/workflows/ci.yml?query=branch%3Amain)
[![docs.rs](https://img.shields.io/docsrs/wol-relay)](https://docs.rs/wol-relay)

A Wake-on-LAN relay server written in Rust.

Provides the ability to wake computers in the subnet of the relay server from other subnets.

## Technical Details

While WoL messages can be embedded in any payload, its most common form is as a 102 byte UDP packet to port 9, which is what this server is listening for.

If WoL messages do not get picked up by this relay server, make sure they are not using a different port, protocol or format.


## Run as binary

```bash
cargo install wol-relay --features cli
```

Then, simply running `wol-relay` will run the relay server.

Use `wol-relay --help` for configuration options.


## Run as docker container

```bash
docker run --rm -ti --network host ghcr.io/finomnis/wol-relay -v
```

## Run via docker-compose

```yml
version: '2'
services:
  wol-relay:
    container_name: wol-relay
    image: ghcr.io/finomnis/wol-relay
    restart: 'unless-stopped'
    network_mode: 'host'
    command:
      - '-v' # Print more logs; repeat for even more
```


## Contributions

Contributions are welcome!

I primarily wrote this crate for my own convenience, so any ideas for improvements are
greatly appreciated.
