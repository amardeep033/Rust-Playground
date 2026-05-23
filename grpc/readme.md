proto/hello.proto
        ↓
build.rs runs tonic_build
        ↓
Rust code generated into target/.../out
        ↓
include_proto! includes it
        ↓
You implement generated trait
        ↓
Server runs
