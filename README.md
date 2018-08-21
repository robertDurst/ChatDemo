# Chat Demo
[![Build Status](https://travis-ci.org/ColbyCypherSociety/ChatDemo.svg?branch=master)](https://travis-ci.org/ColbyCypherSociety/ChatDemo)
A live demonstration of private communication on a public channel.

## Prerequisites
* A recent version of node/npm (probably node 8+ should be fine)
* Nightly Rust (not sure of the latest working version of nightly, but `nightly-2018-07-30` is a safe bet)

## How to Use?

### Setup
```shell
git clone https://github.com/ColbyCypherSociety/ChatDemo.git
cd ChatDemo
npm install
npm run build-debug // or npm run build-release
```

### Start Client
```shell
npm run client
```

### Start Server
```shell
npm run server
```

### Test
```shell
cargo test
```

## References

The underlying crypto makes heavy use of the crates in [rust-num](https://github.com/rust-num), specifically [num-bigint](https://github.com/rust-num/num-bigint).

The randomness for the crypto uses the rust [rand](https://crates.io/crates/rand) crate.
