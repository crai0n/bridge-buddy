# Bridge Buddy

## Introduction
Bridge Buddy is a small app designed to help you evaluate Bridge Hands according to different criteria. Currently implementation is based on the official bidding system of the German Bridge Association [Deutscher Bridge Verband](http://www.dbv.de). In the future, other evaluation systems might be added. We are also planning to expand Bridge Buddy's capabilities to include bidding, evaluating a partner's and maybe even opponents' bids.

## Development Environment
Bridge Buddy is developed exclusively in Rust. To set up your development environment, we recommend using [rustup](http://rustup.sh). 

To deploy our browser-app, we are using [`trunk`](https://trunkrs.dev/).


After basic toolchain-setup, install the WASM-target and `trunk` by running

```{sh}
rustup target add wasm32-unknown-unknown
cargo install trunk
```

You are now ready to clone the Bridge Buddy repository and run our app

```
git clone https://github.com/crai0n/bridge-buddy
cd bridge-buddy
trunk serve --open
```