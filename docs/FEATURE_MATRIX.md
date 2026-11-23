# FEATURE MATRIX — ULTRA STREAM

Esta matriz é o controle oficial de funcionalidades e paridade.

Estados:

- NÃO INICIADO.
- EM IMPLEMENTAÇÃO.
- IMPLEMENTADO-NÃO VALIDADO.
- VALIDADO.
- BLOQUEADO.

| ID | Categoria | Funcionalidade | Fonte/URL | Edição | Dependência/licença | Fase | Estado | Evidência |
|---|---|---|---|---|---|---|---|---|
| US-IN-001 | Input | Câmera/capture card | Pendente | A confirmar | Driver/SDK | 3 | NÃO INICIADO | — |
| US-IN-002 | Input | Arquivo de vídeo | Pendente | A confirmar | Codec/plugin | 6 | NÃO INICIADO | — |
| US-IN-003 | Input | Imagem | Pendente | A confirmar | Decoder | 6 | NÃO INICIADO | — |
| US-IN-004 | Input | Fonte de cor/barras | Requisito próprio | — | Nenhuma prevista | 2 | NÃO INICIADO | — |
| US-IN-005 | Input | Áudio | Pendente | A confirmar | Driver | 5 | NÃO INICIADO | — |
| US-IN-006 | Input | SRT/RTMP/RTSP/HLS | Pendente | A confirmar | Plugins | 6/7 | NÃO INICIADO | — |
| US-IN-007 | Input | NDI/RIST/WebRTC | Pendente | A confirmar | SDK/protocolo | 6/8 | NÃO INICIADO | — |
| US-IN-008 | Input | Desktop/janela | Pendente | A confirmar | API do SO | 6 | NÃO INICIADO | — |
| US-PR-001 | Produção | Preview/Program | Pendente | A confirmar | Motor | 4 | NÃO INICIADO | — |
| US-PR-002 | Produção | Cut/Fade/T-bar | Pendente | A confirmar | GPU | 4 | NÃO INICIADO | — |
| US-CO-001 | Composição | Layers/PIP | Pendente | A confirmar | GPU | 4/6 | NÃO INICIADO | — |
| US-CO-002 | Composição | Chroma/Luma Key | Pendente | A confirmar | GPU | 6 | NÃO INICIADO | — |
| US-AU-001 | Áudio | Mixer e buses | Pendente | A confirmar | Backend áudio | 5 | NÃO INICIADO | — |
| US-AU-002 | Áudio | Peak/RMS | Requisito próprio | — | DSP | 5 | NÃO INICIADO | — |
| US-RE-001 | Gravação | Program recording | Pendente | A confirmar | Codec | 7 | NÃO INICIADO | — |
| US-RE-002 | Gravação | ISO | Pendente | A confirmar | Codec/storage | 9 | NÃO INICIADO | — |
| US-RP-001 | Replay | Replay e buffer circular | Pendente | A confirmar | Memória/storage | 9 | NÃO INICIADO | — |
| US-OU-001 | Saídas | Fullscreen/Clean feed | Pendente | A confirmar | GPU | 8 | NÃO INICIADO | — |
| US-OU-002 | Saídas | DeckLink/AJA | Pendente | A confirmar | SDK fabricante | 8 | NÃO INICIADO | — |
| US-CT-001 | Controle | API HTTP/WebSocket | Requisito próprio | — | Axum candidato | 11 | NÃO INICIADO | — |
| US-CT-002 | Controle | Web Controller | Requisito próprio | — | React/TS candidato | 11 | NÃO INICIADO | — |
| US-AU-003 | Automação | MIDI/Macros | Pendente | A confirmar | Hardware/API | 12 | NÃO INICIADO | — |
| US-PT-001 | PTZ | VISCA/ONVIF | Pendente | A confirmar | Câmera/rede | 12 | NÃO INICIADO | — |
| US-PL-001 | Plugins | Sistema de plugins | Requisito próprio | — | ABI/isolamento | 13 | NÃO INICIADO | — |

| ID | Subsistema | Funcionalidade | Implementação prevista | Dependências | Critério de validação | Estado |
|---|---|---|---|---|---|---|
| INT-001 | Interoperabilidade | ABI C versionada | C + Rust | Contrato ABI | Testes de compatibilidade | Pendente |
| INT-002 | Interoperabilidade | Integração Rust/C++ | Rust + C++ | ABI C | Módulo C++ executado pelo núcleo Rust | Pendente |
| INT-003 | Interoperabilidade | Gestão de memória FFI | Rust + C++ | ABI C | Testes de propriedade e liberação | Pendente |
| INT-004 | Interoperabilidade | Tratamento de erros FFI | C + Rust + C++ | ABI C | Erros propagados corretamente | Pendente |
| INT-005 | Interoperabilidade | Versionamento de módulos | Rust + C | Registro de módulos | Testes de compatibilidade | Pendente |
| INT-006 | Plugins | Registro e descoberta | Rust | Plugin Manager | Descoberta de módulos válidos | Pendente |
| INT-007 | Plugins | Carregamento in-process | Rust + C/C++ | ABI C | Inicialização e encerramento | Pendente |
| INT-008 | Plugins | Execução out-of-process | Rust + C/C++ | IPC | Isolamento e comunicação | Pendente |
| INT-009 | Plugins | Recuperação de falhas | Rust | Supervisão de processos | Núcleo permanece operacional | Pendente |
| HW-001 | Device Manager | API unificada de dispositivos | Rust | Núcleo | Testes de contrato | Pendente |
| HW-002 | Hardware | Adaptador DeckLink | C++ ou Rust | SDK Blackmagic | Teste com hardware real | Pendente |
| HW-003 | Hardware | Adaptador AJA | C++ ou Rust | SDK AJA | Teste com hardware real | Pendente |
| HW-004 | Hardware | Adaptador Bluefish444 | C++ ou Rust | SDK Bluefish444 | Teste com hardware real | Pendente |
| PERF-001 | Desempenho | Benchmark Rust/C++ | Rust + C++ | Ambiente de testes | Relatório reproduzível | Pendente |
| PERF-002 | Desempenho | Benchmark de buffers | Rust + C++ | ABI C | Medição de cópias e latência | Pendente |


## Regra

Na Fase 0, expandir esta tabela para incluir todas as funcionalidades e operações da API.

Cada linha deve possuir fonte oficial e critério de aceitação.

Somente funcionalidades VALIDADO entram no percentual de paridade.

Itens não confirmados devem permanecer identificados como pendentes.