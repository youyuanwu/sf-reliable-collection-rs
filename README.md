# Service Fabric Reliable Collection for Rust
Rust support for [reliable collection](https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-reliable-services-reliable-collections) in Service Fabric.

Experimental and APIs are subject to change.

Currently only Winodws is supported.

See [sample](crates\samples\kvstore) for usage.

# Build
```ps1
cmake . -B build
cmake --build build
cargo build --all
```

# Run Example on SF-onebox
```ps1
.\scripts\kvstore_ctl.ps1 -Action Add
```

# License
MIT

