//! Ultra Stream — interface desktop para Windows.
//! Esqueleto mínimo (sem GUI implementada nesta fase).

mod core_integration;

use ultra_stream_core::CoreState;

fn main() {
    let mut state = CoreState::new();
    state.mark_initialized();

    println!(
        "Ultra Stream desktop esqueleto — estado inicializado: {}",
        core_integration::is_core_initialized(&state)
    );
}
