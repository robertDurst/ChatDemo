# Chat Demo
[![Build Status](https://travis-ci.org/ColbyCypherSociety/ChatDemo.svg?branch=master)](https://travis-ci.org/ColbyCypherSociety/ChatDemo)<br>
A live demonstration of private communication on a public channel. 

<div style="center"><img src="https://imgur.com/1MooWWG.png"></div>

## How it Works

We utilize simple web sockets via [socket.io](http://socket.io/) to create a central chat room that broadcasts all messages to all members and [RSA](https://en.wikipedia.org/wiki/RSA_(cryptosystem)) for message encryption/decryption.

Before joining the chat room, the user generates a public key `[e,n]` and private key `[d,n]`. After generating the keys, the client *Registers* with the chat server by sending its public key which is broadcasted to all users.

Once the user has joined they can send unencrypted or encrypted messages. To send encrypted messages, they can insert a joined user's public key into the *encrypt* field and the message they want to encrypt in the *send* field. After clicking the encrypt button, their message will be encrypted and replace the plaintext in the *send* field. 

Messages are filtered by each client. When a client receives a message with it's public key as the header, it will automatically decrypt the message and alert the user that a message was received.

<div style="center"><img src="https://imgur.com/FsOz0NK.png"></div>

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

**Note:** current client utilizes server running at: https://enigmatic-savannah-85282.herokuapp.com/

### Test
```shell
cargo test
```

## Cryptography Dependencies

The underlying crypto makes heavy use of the crates in [rust-num](https://github.com/rust-num), specifically [num-bigint](https://github.com/rust-num/num-bigint) and [num-trait](https://github.com/rust-num/num-trait) crates.

The randomness for the crypto uses the rust [rand](https://crates.io/crates/rand) crate.
