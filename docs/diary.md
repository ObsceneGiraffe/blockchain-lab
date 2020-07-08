# 2020-07-04

I decided to implement a toy blockchain to teach myself the concepts of blockchain tech.
I want to start with a simple toy example in [Rust][rust-blockchain-tutorial]
to get the gist of creating and adding to a blockchain.

I want to use some libraries that I've been wanting to learn to accomplish this basic blockchain.
[Chrono][chrono-home] will be used for timestamping the blocks, [Bytes][bytes-home] will be used to process the raw bytes and [Ring][ring-home] will be used to SHA-256 the chain together.

I want to then augment this example to save the blockchain to disk.
I'll use [miniserde][mini-serde-home] to serialize the blockchain to and from disk.

I want to be able to fire up a given amount of independant nodes and see them interact.
This will require more research, but in general they will communicate just like bitcoin nodes using TCP.
How the nodes find each other is another [reasearch][satoshi_client_node_discovery] topic, 

A want to be able to monitor each node in the blockchain

[rust-blockchain-tutorial]: https://asymmetric.github.io/2018/02/11/blockchain-rust/
[mini-serde-home]: https://github.com/dtolnay/miniserde
[ring-home]: https://github.com/briansmith/ring
[bytes-home]: https://github.com/tokio-rs/bytes
[chrono-home]: https://github.com/chronotope/chrono
[satoshi_client_node_discovery]: https://en.bitcoin.it/wiki/Satoshi_Client_Node_Discovery

# 2020-07-05

Reasearched more tools for the project: [cargo fuzz][cargo-fuzz-home] and scanned the associated [book][rust-fuzz-book].

Started reading the [protocol][bitcoin-docs-protocol] and [core][bitcoin-docs-core-overview] documentation of Bitcoin to compare it with the [Rust blockchain tutorial][rust-blockchain-tutorial]

[bitcoin-docs-protocol]: https://en.bitcoin.it/wiki/Protocol_documentation
[bitcoin-docs-core-overview]: https://en.bitcoin.it/wiki/Bitcoin_Core_0.11_(ch_1):_Overview
[cargo-fuzz-home]: https://github.com/rust-fuzz/cargo-fuzz
[rust-fuzz-book]: https://rust-fuzz.github.io/book/cargo-fuzz.html

# 2020-07-06

Read the docs on Bitcoin [target][bitcoin-docs-target] and [difficulty][bitcoin-docs-difficulty].

Researched how Substrate represents the [256 bit ints][substrate-uint.rs] for hashing and difficulty.

[substrate-uint.rs]: https://github.com/paritytech/parity-common/blob/master/uint/src/uint.rs
[bitcoin-docs-target]: https://en.bitcoin.it/wiki/Target
[bitcoin-docs-difficulty]: https://en.bitcoin.it/wiki/Difficulty

