# magic_login_app

> Second http-enabled client for amazing opportunity

## Build Setup

``` bash
# install dependencies
$ cargo 


# in case of errors with core-js please install core-js manually
$ npm i -D core-js@2 @babel/runtime-corejs2

# generate new certificates. to do so - please run following command from app root
$ mkdir server/secret_keys
$ cp ../asd_clear_setup/server/secret_keys/asdemo_jwt256.key.pub src/


# serve with hot reload at localhost:3000
$ npm run dev

# build for production and launch server
$ rustup run nightly cargo build --release
$ npm run start

# generate static project
$ npm run generate
```

