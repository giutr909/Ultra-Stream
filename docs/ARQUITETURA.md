# ARQUITETURA DO ULTRA STREAM

## Visão geral

O Ultra Stream será desenvolvido com arquitetura modular.

A aplicação desktop executa o motor audiovisual localmente.

A aplicação web atua como cliente remoto por meio de API HTTP e WebSocket.

## Módulos previstos

- ultra-stream-core
- ultra-stream-media
- ultra-stream-inputs
- ultra-stream-outputs
- ultra-stream-devices
- ultra-stream-compositor
- ultra-stream-audio
- ultra-stream-recording
- ultra-stream-replay
- ultra-stream-streaming
- ultra-stream-automation
- ultra-stream-control-api
- ultra-stream-storage
- ultra-stream-monitoring
- ultra-stream-plugins
- ultra-stream-platform
- ultra-stream-desktop

## Núcleo

O núcleo deve conter:

- ProjectState.
- CommandBus.
- Eventos.
- Configurações.
- Regras de negócio.
- Interfaces dos subsistemas.

A GUI desktop, a API web e os atalhos devem utilizar os mesmos comandos de domínio.

## Motor audiovisual

Responsável por:

- Captura.
- Decodificação.
- Composição.
- Mixagem.
- Gravação.
- Streaming.
- Replay.
- Sincronismo.
- Gerenciamento de dispositivos.

## Vídeo

Preservar timestamps, timebase, formatos, colorimetria, bit depth, HDR e metadados.

Avaliar processamento GPU e formatos de memória adequados.

## Áudio

Avaliar CPAL, buffers pré-alocados, DSP, clocks, buses, medição e sincronismo.

Não executar operações bloqueantes no callback de áudio.

## Desktop

Interface principal de operação profissional.

Deve permitir controle completo dos recursos implementados.

## Web

Aplicação complementar para controle remoto e monitoramento.

Não executar o motor audiovisual no navegador.

## Persistência

Projetos e presets devem possuir schema versionado e migrações.

## Plugins

A arquitetura deve permitir extensibilidade, com ABI versionada ou isolamento de processos, após validação.

## Segurança

API local por padrão.

Acesso remoto deve exigir autenticação e autorização.

Não expor credenciais.

## Observabilidade

Logs estruturados, métricas reais e diagnóstico por subsistema.

## Decisões

Registrar decisões importantes em docs/adr/.


# Arquitetura híbrida Rust, C e C++

## 1. Princípio arquitetural

O Ultra Stream utilizará uma arquitetura híbrida, tendo Rust como linguagem principal e C++ como linguagem complementar de primeira classe.

A escolha entre Rust e C++ deverá considerar os requisitos técnicos de cada componente, a disponibilidade de bibliotecas, SDKs, desempenho medido, estabilidade e facilidade de manutenção.

C não será utilizado como linguagem principal de desenvolvimento, mas como interface de interoperabilidade entre módulos implementados em diferentes linguagens.

A arquitetura não deverá depender da ABI nativa do C++ para comunicação entre componentes independentes.

## 2. Responsabilidades por linguagem

### Rust

Responsável principalmente por:

- Núcleo de gerenciamento e orquestração.
- Gerenciamento de projetos, sessões, cenas e entradas.
- CommandBus e comunicação entre subsistemas.
- Gerenciamento de estado.
- API HTTP e WebSocket.
- Interface desktop.
- Controle remoto e integração com a aplicação web.
- Persistência e configuração.
- Monitoramento, logs e diagnóstico.
- Gerenciamento do ciclo de vida dos módulos.
- Integração com GStreamer e demais bibliotecas quando tecnicamente adequado.

### C++

Responsável por componentes especializados quando houver justificativa técnica:

- Adaptadores para SDKs de fabricantes.
- Integração com APIs nativas C++.
- Captura e saída de vídeo.
- Processamento especializado de áudio e vídeo.
- Componentes de composição e processamento GPU.
- Integração com bibliotecas de terceiros.
- Módulos de alto desempenho.
- Funcionalidades que dependam de bibliotecas com interface C++.

### C

Responsável pela fronteira de interoperabilidade:

- ABI pública dos módulos.
- Tipos e estruturas compatíveis entre linguagens.
- Handles opacos.
- Códigos de erro.
- Contratos de memória e propriedade.
- Funções de inicialização, execução e encerramento.

## 3. Camada de interoperabilidade

A integração entre Rust e C++ será realizada por meio de uma ABI C versionada.

A camada de interoperabilidade deverá definir:

- Versão da ABI.
- Versão individual de cada módulo.
- Identificação única do módulo.
- Inicialização e encerramento.
- Estruturas de dados com tamanho e versão explícitos.
- Handles opacos para recursos internos.
- Códigos de erro padronizados.
- Propriedade e liberação de memória.
- Contratos de concorrência.
- Callbacks e convenções de chamada.
- Compatibilidade entre versões.

Exceções C++ nunca poderão atravessar a fronteira da ABI.

Tipos complexos da STL, classes C++ e objetos dependentes de compilador não poderão ser expostos como contratos públicos.

## 4. Modalidades de execução

### In-process

Os módulos são carregados no mesmo processo do núcleo.

Indicado para componentes confiáveis que necessitam de baixa latência e compartilhamento direto de buffers e recursos.

### Out-of-process

Os módulos são executados em processos independentes.

Indicado para:

- SDKs de terceiros.
- Drivers e adaptadores instáveis.
- Componentes com dependências incompatíveis.
- Módulos que precisam de isolamento de falhas.
- Processamentos que possam comprometer a estabilidade do núcleo.

A comunicação poderá utilizar IPC, memória compartilhada e mecanismos de compartilhamento de recursos GPU.

A modalidade será escolhida conforme os requisitos de latência, segurança e estabilidade.

## 5. Device Manager

O núcleo deverá possuir um Device Manager independente dos fabricantes.

Sua responsabilidade será descobrir, identificar, inicializar, monitorar e gerenciar dispositivos profissionais.

A arquitetura deverá permitir adaptadores independentes para:

- Blackmagic Design DeckLink.
- AJA Video Systems.
- Bluefish444.
- Interfaces profissionais de áudio.
- Dispositivos de captura e saída.
- Futuros fabricantes.

Os adaptadores poderão ser implementados em Rust ou C++, conforme os SDKs e APIs disponíveis.

A existência de um adaptador não significa que o dispositivo esteja validado. O suporte deverá ser comprovado em hardware real ou em testes de conformidade claramente identificados.

## 6. Pipeline audiovisual

O pipeline audiovisual deverá ser independente da linguagem de implementação de seus módulos.

GStreamer será avaliado como backend principal de mídia, enquanto FFmpeg poderá ser utilizado como tecnologia complementar.

Módulos C++ poderão integrar-se ao pipeline por meio de APIs e interfaces suportadas pelas respectivas bibliotecas.

Não criar dependências desnecessárias entre o núcleo Rust e a implementação interna dos módulos C++.

A arquitetura deverá permitir substituir ou atualizar componentes sem exigir reescrita do núcleo.

## 7. Gerenciamento de buffers

A arquitetura deverá minimizar cópias de dados entre módulos.

Sempre que tecnicamente possível, deverão ser utilizados:

- Buffers compartilhados.
- Referências com propriedade definida.
- Filas de buffers.
- Memória compartilhada entre processos.
- Compartilhamento de recursos GPU quando suportado.

O contrato deverá definir claramente quem aloca, quem utiliza e quem libera cada recurso.

Não permitir ponteiros inválidos, uso após liberação ou transferência ambígua de propriedade.

## 8. Compilação

Cargo será o sistema principal do workspace Rust.

CMake será utilizado para componentes C++ quando necessário.

A integração deverá permitir que o build do projeto coordene os componentes nativos e produza artefatos compatíveis.

No Windows, a toolchain MSVC será a referência inicial, sem impedir suporte futuro ao Linux.

Dependências nativas deverão ter suas versões, licenças e requisitos documentados.

## 9. Critérios de escolha entre Rust e C++

A escolha da linguagem será baseada em:

- Benchmarks reproduzíveis.
- Latência.
- Throughput.
- Uso de CPU.
- Uso de GPU.
- Consumo e alocação de memória.
- Disponibilidade de SDKs.
- Estabilidade.
- Segurança.
- Complexidade de manutenção.

Não estabelecer que todo processamento crítico obrigatoriamente será implementado em C++ ou Rust.

A decisão deverá ser registrada em ADR quando afetar significativamente a arquitetura.

## 10. Segurança

Todo uso de FFI deverá ser documentado e revisado.

Em Rust, blocos unsafe deverão ser mínimos e possuir invariantes documentadas.

Em C++, deverão ser observadas as regras de propriedade de memória, tratamento de exceções, concorrência e comportamento definido.

Callbacks de áudio em tempo real não poderão realizar operações bloqueantes, I/O, logging síncrono ou alocações dinâmicas não controladas.
