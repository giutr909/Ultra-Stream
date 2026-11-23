# PERFORMANCE

O Ultra Stream deve ser desenvolvido visando operação profissional em HD e 4K.

Não presumir capacidade antes de benchmark.

Registrar:

- CPU.
- GPU.
- RAM.
- Armazenamento.
- Driver.
- Sistema operacional.
- Resolução.
- FPS.
- Inputs.
- Outputs.
- Codecs.
- Efeitos.
- Duração.

Medir:

- FPS.
- Frames perdidos.
- Latência.
- Jitter.
- CPU.
- GPU.
- Memória.
- Bitrate.
- Perda de pacotes.
- Buffer.
- Drift.
- Underruns.

Executar testes de carga e operação prolongada.

Metas quantitativas devem ser definidas a partir de hardware de referência e resultados medidos.



# Desempenho e otimização híbrida

## 1. Princípio

O Ultra Stream deverá priorizar desempenho previsível, baixa latência, estabilidade e uso eficiente dos recursos de hardware.

Rust será a linguagem principal. C++ poderá ser utilizado em componentes especializados quando os resultados técnicos justificarem sua adoção.

A linguagem, isoladamente, não será considerada evidência de desempenho.

## 2. Avaliação por componente

Cada componente relevante poderá possuir implementações alternativas em Rust e C++ para comparação.

Os benchmarks deverão manter as mesmas condições de entrada, processamento, saída e hardware.

## 3. Métricas obrigatórias

- Latência média.
- P95 e P99 de latência.
- Throughput.
- Uso de CPU.
- Uso de GPU.
- Memória residente.
- Quantidade de alocações.
- Quantidade de cópias de buffers.
- Estabilidade sob carga prolongada.

## 4. Caminhos críticos

Os caminhos críticos de áudio e vídeo deverão evitar operações bloqueantes.

Callbacks de áudio em tempo real não poderão realizar:

- I/O.
- Logging síncrono.
- Esperas bloqueantes.
- Locks não apropriados para tempo real.
- Alocações dinâmicas não controladas.

A interoperabilidade Rust/C++ deverá evitar conversões e cópias desnecessárias.

## 5. GPU e buffers

A arquitetura deverá permitir compartilhamento de recursos GPU quando suportado pelas APIs e plataformas envolvidas.

Não assumir que um buffer é compartilhável entre APIs ou processos sem validar os mecanismos de interoperabilidade.

## 6. Decisões

Toda otimização significativa deverá registrar:

- Problema observado.
- Cenário de reprodução.
- Implementação original.
- Alternativa avaliada.
- Resultados medidos.
- Custo de manutenção.
- Decisão final.
