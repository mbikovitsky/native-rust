# native-rust

A Rust PoC for writing a native Windows executable. Based on Pavel Yosifovich's
[talk][native-talk] and [code][native-code].

Project configuration is based on [this][freestanding] guide.

Build with `cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:native"`.

[native-talk]: https://www.youtube.com/watch?v=EKBvLTuI2Mo
    "Native Applications: What, Why, and How? - YouTube"

[native-code]: https://github.com/zodiacon/NativeApps
    "zodiacon/NativeApps: Demos and presentation from SECArmy Village Grayhat 2020"

[freestanding]: https://os.phil-opp.com/freestanding-rust-binary/
    "A Freestanding Rust Binary | Writing an OS in Rust"
