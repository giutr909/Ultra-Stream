//! Integração mínima com o núcleo (substituto, sem funcionalidade de produto).
//!
//! Este módulo existe apenas para permitir que o binário desktop compile
//! dependendo de `ultra-stream-core` sem implementar subsistemas.

use ultra_stream_core::CoreState;

/// Verifica se o núcleo está inicializado (placeholder).
pub fn is_core_initialized(state: &CoreState) -> bool {
    state.initialized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_core_initialized_returns_false_for_default_state() {
        let state = ultra_stream_core::CoreState::new();
        assert!(!is_core_initialized(&state));
    }
}
