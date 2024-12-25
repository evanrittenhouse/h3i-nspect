# Introduction 

h3i-nspect is a server-side HTTP/3 conformance test suite, built on top of [`h3i`](https://docs.rs/h3i). 

This is currently in a very rough state, and merely serves to provide scaffolding and a harness to report tests. 

# Usage

```shell
cd h3i-nspect
cargo run -- https://cloudflare-quic.com
```

# To-Do

1. More test cases (obviously)
2. Async sections. Intra-section tests are async, but the sections are simply looped over which is wrong
        - Hand-in-hand with this, adding a single async progress bar and reporting failed tests at the end of the run would be helpful rather than prnting everything
