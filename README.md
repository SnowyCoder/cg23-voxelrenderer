## Voxel Renderer

This is my entry for the "Mobile Programming" exam developed in 2023 for UniMoRe.
The initial files are based on ![rust-mobile examples](https://github.com/rust-mobile/rust-android-examples/tree/main/agdk-winit-wgpu),
while using [learn-wgpu](https://github.com/sotrh/learn-wgpu) to, well, learn wgpu.

## Features
- Instance-based rendering
- .vly format parsing
- .vox format parsing
- Blinn-Phong shader
- Android & Desktop support
- Runtime texture palette generation


## Not implemented (yet)
- Face merging
- Raytracing
- Web support
- Transparency


## Libraries
Original project requirements specify it's not possible to use any additional libraries like game engines or rendering abstraction layers.
Here's a list of the used libraries and their purpose
- `log`, `env_logger`, `android_logger`: Log handling helpers
- `winit`: Window initialization on multiple platforms
- `wgpu`: WebGPU-like interface and implementation
          This is the only library that could be described as an abstraction layer, but the project has already been discussed in person with the professor
- `cgmath`: Linear Algebra helper
- `ply-rs`: .ply  format parser
- `bytemuck`: Helper for serializing data to send to the GPU
- `nom`: Parser framework (for .vox and .vly formats)
- `anyhow`: Error handling helper
- `pollster`: Very lightweight async runtime
- `jni`: Java Native Interface library, used to retrieve the model to render from Android


## What is vly format?
A simple voxel serialization format similar to ply but simpler, developed specifically as a challenge for this project.
You can find more precise requirements in [ProgettoCG2324.pdf](ProgettoCG2324.pdf)

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

Note: add `--release` in cargo parameters to enable compiler optimizations
