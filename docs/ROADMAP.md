# ROADMAP — ULTRA STREAM

Todas as fases exigem build, testes, evidências, relatório e aprovação humana.

## Fase 0 — Governança e auditoria

- Inspecionar repositório.
- Verificar Git.
- Inventariar ambiente.
- Completar matriz de funcionalidades.
- Verificar licenças.
- Registrar decisões.
- Confirmar arquitetura inicial.

Portão: documentação e matriz aprovadas.

## Fase 1 — Fundação Rust e GUI

- Workspace Rust.
- Build.
- Logs.
- Diagnóstico.
- Janela desktop mínima.

Portão: aplicação abre e fecha corretamente.

## Fase 2 — Modelo de mídia

- Tipos de vídeo.
- Tipos de áudio.
- Timestamps.
- Clocks.
- Fontes sintéticas reais.

Portão: pipeline sintético validado.

## Fase 3 — Captura e dispositivos

- Enumeração.
- Capacidades.
- Hot-plug.
- Captura real.
- Preview.

Portão: câmera e microfone reais validados.

## Fase 4 — Compositor

- Preview.
- Program.
- Cut.
- Fade.
- T-bar.
- Fade to Black.
- Layers.
- Composição GPU.

Portão: transições e composição testadas.

## Fase 5 — Áudio

- Mixer.
- Faders.
- Buses.
- Master.
- VU.
- Monitoramento.
- Sincronismo.

Portão: testes com sinal conhecido e dispositivo real.

## Fase 6 — Inputs e filtros

- Arquivos.
- Imagens.
- Cor.
- Barras.
- Browser.
- Desktop.
- Streams.
- Chroma key.
- LUT.
- Layers.

Portão: tipos de input validados.

## Fase 7 — Gravação e streaming

- Program recording.
- RTMP.
- SRT.
- Múltiplos destinos.
- Reconexão.
- Métricas.

Portão: gravação e recepção real validadas.

## Fase 8 — Outputs

- Fullscreen.
- Clean feed.
- SDI.
- NDI.
- Saídas independentes.

Portão: saídas validadas em hardware ou serviço real.

## Fase 9 — Replay e ISO

- Buffer circular.
- Replay.
- Marcação.
- Slow motion.
- Gravação ISO.

Portão: arquivos e sincronismo validados.

## Fase 10 — Gráficos

- Títulos.
- Lower thirds.
- Placares.
- Templates.
- Animações.
- Fontes de dados.

Portão: gráficos validados na GUI.

## Fase 11 — Controle web

- API.
- WebSocket.
- Autenticação.
- Web Controller.
- Monitoramento.

Portão: controle web altera o estado real do motor.

## Fase 12 — Automação

- MIDI.
- Macros.
- PTZ.
- VISCA.
- ONVIF.
- Tally.

Portão: integrações reais validadas.

## Fase 13 — Plugins

- Manifest.
- SDK.
- Compatibilidade.
- Isolamento.
- Extensibilidade.

Portão: plugin de exemplo validado.

## Fase 14 — Desempenho

- 4K.
- Profiling.
- CPU.
- GPU.
- Memória.
- Latência.
- Soak test.

Portão: relatório com métricas medidas.

## Fase 15 — UAT e distribuição

- Instalação limpa.
- Testes de usuário.
- Empacotamento.
- Licenças.
- SBOM.
- Atualização.
- Recuperação.

Portão: UAT aprovado.



## Fase — Fundação da interoperabilidade Rust/C++

Objetivo: preparar a infraestrutura para módulos nativos escritos em Rust, C e C++.

Entregáveis:

- Definição da ABI C versionada.
- Especificação de ciclo de vida dos módulos.
- Contrato de propriedade de memória.
- Sistema de identificação e versionamento de módulos.
- Protótipo de integração Rust/C.
- Protótipo de integração Rust/C++.
- Integração do CMake ao workspace Cargo.
- Testes de interoperabilidade.
- Documentação das limitações de ABI.
- ADR sobre estratégia de carregamento dos módulos.

Critérios de conclusão:

- Compilação de um módulo C++ independente.
- Carregamento e comunicação pelo núcleo Rust.
- Inicialização e encerramento controlados.
- Tratamento de erros.
- Testes de propriedade de memória.
- Testes de compatibilidade de ABI.
- Build reproduzível no Windows.

Não iniciar a implementação de plugins de produção antes da validação dessa fundação.

## Fase — Sistema de módulos e plugins nativos

Objetivo: permitir a expansão do sistema por módulos especializados.

Entregáveis:

- Registro de módulos.
- Descoberta de módulos.
- Identificação e versionamento.
- Gerenciamento do ciclo de vida.
- Carregamento e descarregamento controlados.
- Validação de compatibilidade.
- Isolamento de módulos quando necessário.
- Relatórios de falhas.
- Documentação para desenvolvimento de módulos.

Os módulos poderão ser escritos em Rust ou C++ e deverão respeitar a ABI pública.

## Fase — Device Manager e integração de hardware

Objetivo: criar uma camada unificada para dispositivos profissionais.

Entregáveis:

- API do Device Manager.
- Descoberta de dispositivos.
- Gerenciamento de entradas e saídas.
- Ciclo de vida dos adaptadores.
- Tratamento de desconexões.
- Recuperação de falhas.
- Adaptador inicial de referência.
- Arquitetura para DeckLink, AJA e Bluefish444.

A implementação de cada fabricante dependerá da disponibilidade de SDK, documentação, licença e hardware de teste.

Não declarar suporte validado sem testes reais.

## Fase — Benchmark e otimização híbrida

Objetivo: identificar quais componentes se beneficiam de implementações especializadas em C++.

Entregáveis:

- Benchmarks Rust.
- Benchmarks C++.
- Comparação de latência.
- Comparação de throughput.
- Medição de consumo de CPU e memória.
- Medição de cópias de buffers.
- Avaliação de integração GPU.
- Relatório técnico de decisão.

O resultado deverá orientar a escolha da linguagem por componente, sem estabelecer preferência automática por C++ ou Rust.


## Regra obrigatória

Ao terminar cada fase, parar e aguardar PASS ou FAIL do responsável humano.