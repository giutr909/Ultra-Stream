
# Interoperabilidade Rust e C++

## 1. Objetivo

Definir a integração entre Rust e C++ no Ultra Stream, permitindo desenvolver módulos nativos sem comprometer a estabilidade do núcleo.

## 2. Modelo

Rust controla o ciclo de vida e a comunicação dos módulos.

C++ implementa funcionalidades especializadas.

Uma ABI C versionada estabelece o contrato entre as linguagens.

## 3. Estrutura conceitual

ultra-stream/
├── crates/
│   ├── ultra-stream-core/
│   ├── ultra-stream-desktop/
│   └── ultra-stream-ffi/
├── native/
│   ├── common/
│   ├── cpp-example/
│   └── device-adapters/
├── include/
│   └── ultra_stream/
│       └── plugin_api.h
└── docs/
    └── INTEROPERABILIDADE_RUST_CPP.md

A estrutura é uma proposta inicial. Sua adoção depende da validação da arquitetura e do sistema de build.

## 4. Contrato

O contrato público deverá utilizar tipos C compatíveis.

Não expor classes C++, templates, STL ou exceções.

Utilizar handles opacos para objetos internos.

Definir propriedade de memória e responsabilidade de liberação.

## 5. Build

Cargo será responsável pelo workspace Rust.

CMake será responsável pelos componentes C++ quando necessário.

A integração entre ambos deverá ser automatizada e reproduzível.

## 6. Segurança

Toda fronteira FFI deverá validar entradas, tamanhos e estados.

O código Rust unsafe deverá ser restrito e documentado.

Exceções C++ deverão ser capturadas dentro do próprio módulo.

## 7. Decisões pendentes

As seguintes decisões exigem protótipos:

- Biblioteca ou mecanismo de carregamento dinâmico.
- Uso de CMake integrado ao Cargo.
- Formato definitivo do contrato ABI.
- Estratégia de versionamento.
- Compartilhamento de buffers.
- IPC para módulos isolados.
- Compatibilidade MSVC e toolchains Linux.

Nenhuma dessas decisões deverá ser tratada como definitivamente aprovada antes da avaliação técnica.
