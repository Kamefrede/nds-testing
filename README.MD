# nds-rs usage example

This is a fairly simple Hello World example that illustrates how to use nds-rs (as of the time of writing this, nds-sys because nds-rs is kinda empty) and how to build a playable NDS file

Keep in mind that you need to have the appropriate Devkitpro packages for NDS installed and your path variables set.

First, start by compiling the code with
```rust
cargo +nightly build --release -Z build-std=core,alloc --target thumbv5te-none-eabi.json
```
for a release build or 
```rust 
cargo +nightly build -Z build-std=core,alloc --target thumbv5te-none-eabi.json
```
for a debug build.


Next(At some point I'll write a script that automates this), go to the `thumbv5te-none-eabi/release` or `thumbv5te-none-eabi/debug` (depending on what type of build you did on the previous step) subdirectory inside the `target` directory and run the following command on your terminal:

```shell script
ndstool -c nds_testing_rs.nds -9 nds-testing -b /opt/devkitpro/libnds/icon.bmp "hello_world_rust;built with devkitARM & rust;http://devkitpro.org"
```
(Keep in mind you need to have the ndstool binary that's provided by devkitpro on your path)
It will then generate a .nds file on the folder you ran the command on. 

... And you're done!

# Credits
https://github.com/rrohrer/rs-nds

https://github.com/rrohrer/crawler
