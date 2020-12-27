# Development Notes

## Logging

look at `tracing` // and `logging` crates

## ES CQRS
- write a leveldb impl
- write a journaldb impl
- write an etcd impl

## CIDR Structs to use

Looking for a library that is on par with Python's `ipaddress`. 

Research on which CIDR library to use
- [ipnetwork](https://github.com/achanda/ipnetwork/) has the most feature rich set of methods.
  - has iterators
  - has containment checks
  - has IPv4 and IPv6
  - clean
- [netaddr2](https://github.com/rye/rust-netaddr2) look to be stale for 12 months. Has a few yet-to-be implemented features
  - has no iterators
  - has containment checks
  - has IPv4 and IPv6
  - clean
- [ipnet]()
  - `ipnet = { version = "2", features = ["serde"] }`
  - Supports IPv4 and IPv6

- [rust-cidr](https://github.com/stbuehler/rust-cidr)
   - not a lot of activity - needs work

## Patricia Trie

Looking for an equivalent to Pythons `pytricia`
- The best is to use trie-db from Paritytech
- `radix` is a model - but it is too immature  - https://github.com/refraction-networking/radix

## Feature Ideas
- Write a Schema
- Validation of Attributes
- Complex Queries for locating

