# ESTRATÉGIA DE TESTES

## Categorias

- Unitários.
- Integração.
- E2E.
- Hardware.
- Audiovisual.
- Performance.
- Soak.
- Regressão.
- Compatibilidade.
- UAT.

## Vídeo

Verificar:

- FPS real.
- Frames descartados.
- Jitter.
- PTS.
- Resolução.
- Pixel format.
- Colorimetria.
- Latência.

Utilizar fontes sintéticas reais e padrões identificáveis.

## Áudio

Verificar:

- Sample rate.
- Peak.
- RMS.
- Ganho.
- Mute.
- Buses.
- Clipping.
- Underruns.
- Overruns.
- Sincronismo.

Utilizar sinais de amplitude conhecida.

## Sincronismo A/V

Gerar eventos de áudio e vídeo alinhados e medir o offset resultante.

## Evidências

Armazenar em evidence/phaseN/.

Não declarar aprovação sem execução.


# Testes de interoperabilidade Rust, C e C++

## 1. Objetivo

Validar a comunicação entre componentes desenvolvidos em Rust, C e C++, garantindo compatibilidade, segurança de memória, estabilidade e desempenho.

## 2. Testes de ABI

Validar:

- Versão da ABI.
- Tamanho das estruturas.
- Alinhamento de memória.
- Convenção de chamada.
- Compatibilidade de tipos.
- Identificação de módulos.
- Compatibilidade entre versões suportadas.
- Rejeição de módulos incompatíveis.

## 3. Testes de ciclo de vida

Validar:

- Criação do módulo.
- Inicialização.
- Execução.
- Pausa.
- Reinicialização.
- Encerramento.
- Destruição.
- Repetição de ciclos.
- Falhas durante a inicialização.

Nenhum módulo poderá deixar recursos ou threads sem gerenciamento.

## 4. Testes de memória

Validar:

- Alocação.
- Liberação.
- Transferência de propriedade.
- Referências inválidas.
- Uso após liberação.
- Buffers vazios.
- Buffers grandes.
- Alinhamento.
- Acesso concorrente.
- Vazamentos de memória.

Utilizar ferramentas de diagnóstico disponíveis para cada plataforma.

## 5. Testes de exceções e erros

Validar:

- Exceções internas de C++.
- Conversão de exceções em códigos de erro.
- Erros de inicialização.
- Erros de execução.
- Falhas de comunicação.
- Módulos incompatíveis.
- Falhas de encerramento.

Exceções nunca poderão atravessar a ABI C.

## 6. Testes de concorrência

Validar:

- Execução paralela de módulos.
- Acesso concorrente a buffers.
- Encerramento durante execução.
- Cancelamento de operações.
- Deadlocks.
- Condições de corrida.
- Thread safety.

## 7. Testes de isolamento

Para módulos out-of-process, validar:

- Comunicação IPC.
- Encerramento inesperado.
- Reinicialização.
- Timeout.
- Corrupção de mensagens.
- Recuperação do núcleo.
- Isolamento de falhas.

Uma falha em um processo auxiliar não poderá derrubar o núcleo principal.

## 8. Testes audiovisuais

Validar módulos integrados ao pipeline audiovisual:

- Continuidade dos buffers.
- Formatos de mídia.
- Timestamps.
- Sincronização de áudio e vídeo.
- Latência.
- Throughput.
- Integridade de frames.
- Desempenho sob carga.

Fontes sintéticas deverão ser identificadas como sintéticas. Não poderão ser apresentadas como captura real de hardware.

## 9. Testes de desempenho

Comparar implementações Rust e C++ equivalentes.

Medir:

- Latência média.
- Latência de percentis elevados.
- Throughput.
- Uso de CPU.
- Uso de memória.
- Quantidade de cópias.
- Alocações.
- Comportamento sob carga prolongada.

Não aceitar conclusões de desempenho sem resultados reproduzíveis.

## 10. Critérios de aprovação

A integração somente será considerada validada quando:

- Os testes de ABI forem aprovados.
- O ciclo de vida dos módulos funcionar corretamente.
- Não forem encontrados erros de memória conhecidos nos cenários testados.
- Os erros forem propagados corretamente.
- Os testes de concorrência forem aprovados.
- Os testes de isolamento forem aprovados quando aplicáveis.
- O build for reproduzível.
- Os resultados forem documentados.

Testes não executados deverão permanecer identificados como pendentes.
