# RandomX, in Rust

RandomX is a proof of work algorithm based on random code execution.
The reference implementation is available on
[GitHub](https://github.com/tevador/RandomX).

This implementation in Rust aims to follow the specification described in the
original implementation. The commit hash used to mimick this implementation is
[89aba80](https://github.com/tevador/RandomX/commit/89aba8092595f18946f5b30bf5d84749b69f13e4).

Interesting articles regarding analysis of RandomX:
- [Trails of bits](https://blog.trailofbits.com/2019/07/02/state/)


## Install

There is no release, yet.

Clone the repository and use:
```shell
cargo build --release
```

### Tests

```shell
cargo install cargo-nextest --locked
cargo nextest run --release
```

### Documentation

```shell
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
```

### Clippy

```shell
cargo clippy --all-features --all-targets --tests -- -W clippy::all -D warnings
```

## Plan

This is a work in progress.

The work is splitted in different tasks, and "person-hour" work is assigned for
each of them. An expected person-hour work will be assigned when the task
will begin in the near future. When the task is completed, the real
"person-hour" work will be added.

The task includes the understanding of C++ reference implementation, including
tests and documentation.
Contributions to the reference implementation are not excluded, like additional
testing or documentation.

A "person-day" work consists of 8 "person-hours" work.
A "person-week" work consists of 5 "person-days" work.

Here the different steps (can evolve)
- [x] Starting reading the documentation, implement primitives defines in
      section 3, setup repository and organization.
  - **Expected**: 2 person-days work (not planned beforehand)
  - **Real**: 2 person-days work.
- [ ] Implementation of the Virtual Machine
  - [ ] Compiling the program
  - [ ] Implement each instruction
  - **Expected**: 2 person-weeks work
  - **Real**: TBD
- [ ] Understand and implement SuperscalarHash
  - **Expected**: 1 person-week work
  - **Real**: TBD
- [ ] TBD
- [ ] Machine optimisations

The final plan would be to make a C API and try to run (the reference implementation of a)[Monero
node](https://github.com/monero-project/monero).

## Contact

- Hayashi Makoto: HayashidaMakoto@proton.me


Privacy is normal.
