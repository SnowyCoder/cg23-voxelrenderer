## Voxel Renderer

This is my entry for the "Mobile Programming" exam developed in 2023 for UniMoRe.
The initial files are based on ![rust-mobile examples](https://github.com/rust-mobile/rust-android-examples/tree/main/agdk-winit-wgpu),
while using [learn-wgpu](https://github.com/sotrh/learn-wgpu) to, well, learn wgpu.

## TODO: SAY A BIT MORE


## What is vly format?
A simple voxel serialization format similar to ply but simpler

## Build Instructions


Android:
```bash
export ANDROID_NDK_HOME="path/to/ndk"
export ANDROID_HOME="path/to/sdk"

rustup target add aarch64-linux-android
cargo install cargo-ndk

cargo ndk -t arm64-v8a -o app/src/main/jniLibs/  build
./gradlew build
./gradlew installDebug
adb shell am start -n co.realfit.agdkwinitwgpu/.MainActivity
```

Desktop:
```bash

cargo run --features desktop -- models/christmas.vly
```