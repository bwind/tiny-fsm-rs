# Tiny finite state machine written in Rust 🦀

This project demonstrates a simple finite state machine implementation in Rust. An FSM describes the behavior of a system that can be in one of a finite number of states, and transitions between those states based on input events.

## TODO

- #[fsm] proc macro
- use hashbrown crate (use criterion)
- guards
- hooks (on_enter, on_exit, on_transition)
- graphviz (.to_dot())
- async transitions (async-trait)
- anyhow & thiserror
