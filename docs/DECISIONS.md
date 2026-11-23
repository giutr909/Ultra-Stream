
# Registro de decisões do Ultra Stream

Este documento registra decisões arquiteturais importantes.

Cada decisão deve possuir:

- Identificador.
- Data.
- Contexto.
- Decisão.
- Justificativa.
- Alternativas consideradas.
- Consequências.
- Estado: proposta, aprovada, rejeitada ou substituída.

## ADR-001 — Rust como linguagem principal

Estado: Aprovada como direção arquitetural.

Rust será a linguagem principal do núcleo do Ultra Stream.

## ADR-002 — Suporte oficial a C++

Estado: Aprovada como direção arquitetural.

C++ será uma linguagem complementar de primeira classe, principalmente para componentes especializados e integração com SDKs nativos.

## ADR-003 — ABI C

Estado: Direção arquitetural aprovada; especificação técnica pendente.

A interoperabilidade entre Rust e C++ deverá utilizar uma interface C versionada.

## ADR-004 — Backend audiovisual

Estado: Em avaliação.

GStreamer será avaliado como backend principal, com FFmpeg como tecnologia complementar.

A decisão definitiva dependerá de protótipos e validações.

## ADR-005 — Interface desktop e web

Estado: Direção arquitetural aprovada.

A GUI desktop será a interface principal de operação. A aplicação web será complementar, para controle e monitoramento remoto.
