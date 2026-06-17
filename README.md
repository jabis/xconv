X4 Converter Rust wrapper for the main functions of XRConvertersMain.exe v0.2.1

### BUILDING
you'll need to build this for the 32-bit target and then copy the resulting xconv.exe to this root folder where the XRConverters.dll resides, until I get around managing a 64-bit dll included, or an installer setup

#### Install the 32-bit compiler target
`rustup target add i686-pc-windows-msvc`

#### Build your project specifically for 32-bit
`cargo build --target i686-pc-windows-msvc --release`