# Ultra Stream

Software profissional de produção audiovisual ao vivo, gratuito e de código aberto.

## Objetivo

Construir uma plataforma de produção ao vivo com interface desktop para Windows e aplicação web complementar para controle remoto.

O processamento audiovisual é realizado localmente pelo motor desktop.



## Tecnologias e arquitetura

O Ultra Stream utiliza uma arquitetura híbrida:

- **Rust:** núcleo principal, gerenciamento, interface desktop, API e orquestração.
- **C++:** módulos audiovisuais especializados, integração com SDKs e hardware profissional e processamento nativo quando tecnicamente justificado.
- **C:** interface ABI versionada para interoperabilidade entre módulos.
- **GStreamer:** backend audiovisual principal candidato.
- **FFmpeg:** tecnologia complementar para processamento e compatibilidade de mídia.
- **wgpu:** tecnologia candidata para composição e processamento GPU.
- **egui/eframe:** tecnologia candidata para interface desktop.
- **Axum e WebSocket:** tecnologia candidata para API e controle remoto.
- **React e TypeScript:** tecnologia candidata para interface web.

As tecnologias ainda em avaliação serão confirmadas por protótipos e decisões arquiteturais documentadas.

- Rust.
- GStreamer.
- FFmpeg.
- wgpu.
- egui/eframe.
- CPAL.
- Axum.
- React.
- TypeScript.
- SQLite.

As tecnologias ainda estão sujeitas à validação por protótipos e testes.



## Arquitetura

O projeto é modular e dividido em subsistemas independentes para captura, composição, áudio, gravação, streaming, replay, automação, dispositivos e controle.

## Desenvolvimento

O desenvolvimento é coordenado pelo OpenCode e pode utilizar REA e agentes especializados quando efetivamente disponíveis.

Consultar AGENTS.md antes de qualquer alteração.

## Estado atual

Fundação documental e workspace inicial.

Nenhuma funcionalidade audiovisual deve ser considerada validada sem evidências.

## Licença

GPL-3.0-or-later é a proposta inicial, sujeita à revisão de compatibilidade das dependências.