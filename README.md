[![Rust CI](https://github.com/stbuehler/rust-cidr/actions/workflows/rust.yml/badge.svg?branch=master&event=push)](https://github.com/stbuehler/rust-cidr/actions/workflows/rust.yml)
[![AppVeyor Status](https://ci.appveyor.com/api/projects/status/m37iv2rue63lamfd?svg=true)](https://ci.appveyor.com/project/stbuehler/rust-cidr)
[![crates.io](https://img.shields.io/crates/v/cidr.svg)](https://crates.io/crates/cidr)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

# cidr crate

This library provides types to represent an IP network (`Cidr`) or an IP
host withing a network (`Inet`).

The naming follows the names of the [PostgreSQL data types](https://www.postgresql.org/docs/current/static/datatype-net-types.html)

The documentation for `master` is located at [https://stbuehler.github.io/rustdocs/cidr/cidr/](https://stbuehler.github.io/rustdocs/cidr/cidr/); released versions are documented at [https://docs.rs/cidr](https://docs.rs/cidr).

## Examples

- example making good use of the `bitstring` feature and the `bitstring-trees` crate: https://github.com/stbuehler/rust-gather-cidr-map
