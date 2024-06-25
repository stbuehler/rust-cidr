# Change Log

## [0.2.3] - 2024-06-25

- `parsers` module:
    * `inet_addr` compatible parser
    * parser combinators to build custom parsing
    * "short IPv4 address" parser (incompatible with `inet_addr`).
- short IPv4 address support in `FromStr` implementations is deprecated and
  going to be removed in 0.3.0
- `no_unsafe` feature to use safe but slower fallbacks
- `overflowing_add` and `overflowing_sub` methods on `Inet` types

## [0.2.2] - 2023-06-25

- Make all functions const if possible

## [0.2.1] - 2021-10-08

- Fix major problems in the `BitString` trait implementation for `AnyIpCidr`

## [0.2.0] - 2021-08-09

### Added

- Implement `IntoIterator` for `IpCidr` structs
- Support (and require) various default traits
- `InetPair`: pair of two addresses in the same network
- Implement "trait" functions directly on data types and only forward in trait impl (#6); this makes it easier to use datastructs without traits in scope.

### Changed

- Made `bitstring` dependency optional. Use `cidr = { version = "0.2.0", features = [ "bitstring" ] }` to enable it.
- Removed `serde` from default features. Use `cidr = { version = "0.2.0", features = [ "serde" ] }` to enable it.
- Added `std` feature; enabled by default.  Also mandatory for now as `std::net` isn't available in `core`.
- Seal trait implementations
- Renamed `Inet::next` to `Inet::increment`
- Iterators (on `IpCidr` structs) now return `Inet` items instead of flat addresses. Use `.addresses()` on the iterator type to only iterate over the addresses.

[0.2.0]: https://github.com/stbuehler/rust-cidr/compare/cidr-0.1.1...cidr-0.2.0
[0.2.1]: https://github.com/stbuehler/rust-cidr/compare/cidr-0.2.0...cidr-0.2.1
[0.2.2]: https://github.com/stbuehler/rust-cidr/compare/cidr-0.2.1...cidr-0.2.2
[0.2.3]: https://github.com/stbuehler/rust-cidr/compare/cidr-0.2.2...cidr-0.2.3
