# Rust Simple Greetings server for magic_login_app

> Second http-enabled client for amazing opportunity
> Please make sure you have rustup (<https://rustup.rs>) installed with nightly toolchain 
$ rustup toolchain install nightly
$ rustup default nightly
## Build Setup

``` bash
# Clone this repository and cd to resulting dirrectory
$ git clone https://github.com/1mehal/rust_simple_server.git
$ cd rust_simple_server
# copy generated public key from application server (I suppose it in folder higher)
$ cp ../asd_clear_setup/server/secret_keys/asdemo_jwt256.key.pub src/

# run server in development mode 
$ cargo run

# build for production and launch server
$ rustup run nightly cargo build --release
```

