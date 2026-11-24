//! Ultra Stream — núcleo principal do domínio.
//!
//! Este módulo constitui a fonte única de verdade do estado do projeto.
//! Nenhum código de processamento audiovisual é implementado nesta fase.

/// Resultado genérico de operações do núcleo.
pub type CoreResult<T> = Result<T, CoreError>;

/// Erros do núcleo do Ultra Stream.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("módulo não inicializado")]
    ModuleNotInitialized,
    #[error("operação não suportada: {0}")]
    NotSupported(String),
}

/// Estado mínimo de substituto do núcleo.
#[derive(Debug, Default)]
pub struct CoreState {
    pub initialized: bool,
}

impl CoreState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark_initialized(&mut self) {
        self.initialized = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_is_not_initialized() {
        let state = CoreState::new();
        assert!(!state.initialized);
    }

    #[test]
    fn mark_initialized_sets_flag() {
        let mut state = CoreState::new();
        state.mark_initialized();
        assert!(state.initialized);
    }
}
