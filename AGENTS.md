# AGENTS.md — PROJETO ULTRA STREAM

# 1. IDENTIDADE E MISSÃO DO PROJETO

O Ultra Stream (Nome provisorio) é um software profissional de produção audiovisual ao vivo, gratuito e de código aberto, desenvolvido com o objetivo de alcançar paridade funcional progressiva com softwares profissionais de produção ao vivo, tendo como referência de funcionalidades o vMix 4K Pro.

O projeto deve possuir uma arquitetura própria, independente e modular, sem copiar código, interface proprietária, identidade visual, assets, textos ou implementação interna de outros softwares.

## 1.1 Objetivos fundamentais

- Desenvolver uma aplicação desktop para Windows como interface principal de operação.
- Desenvolver uma aplicação web complementar para controle remoto, configuração e monitoramento.
- Executar o processamento audiovisual localmente no motor desktop.
- Permitir operação profissional de vídeo, áudio, composição, gravação, streaming e automação.
- Desenvolver uma arquitetura modular e expansível.
- Utilizar Rust como linguagem principal.
- Preparar a arquitetura para suporte futuro a Linux.
- Permitir desenvolvimento colaborativo entre o responsável humano, REA e agentes de inteligência artificial.
- Manter o projeto sustentável, documentado, testável e preparado para atualizações futuras.

## 1.2 Plataforma e tecnologias

Diretrizes do projeto:

- Linguagem principal: Rust.
- Sistema operacional inicial: Windows 10/11 x64.
- Sistema operacional futuro: Linux.
- Backend audiovisual principal candidato: GStreamer.
- Ferramenta complementar de mídia: FFmpeg.
- Composição gráfica GPU: wgpu/WGSL, sujeita a validação.
- Interface desktop: egui/eframe, sujeita a validação.
- Áudio: CPAL e implementação ou bibliotecas DSP avaliadas.
- API: Axum e WebSocket, sujeitos a validação.
- Interface web: React e TypeScript.
- Persistência: SQLite.
- Serialização: Serde.
- Observabilidade: tracing e métricas estruturadas.

Essas tecnologias são candidatas ou diretrizes. Nenhuma biblioteca deve ser considerada tecnicamente validada antes de protótipos, testes e análise de compatibilidade.

# 2. HIERARQUIA DE AUTORIDADE

A hierarquia de autoridade do projeto é:

1. Responsável humano pelo projeto.
2. GuitHub Copilot, Claude e O penCode como agentes coordenadores.
3. REA,Ghidra, SysSysPy quando disponível e efetivamente integrado.
4. Agentes especializados de inteligência artificial.
5. Ferramentas automatizadas de build, testes e análise.

O responsável humano mantém a autoridade final sobre arquitetura, escopo, decisões importantes, licenciamento, alterações destrutivas e aprovação das fases.

O OpenCode coordena o trabalho, delega tarefas, revisa resultados e integra as alterações.

REA deve ser utilizado quando estiver realmente disponível e suas capacidades forem conhecidas. Nunca presumir integração, ferramentas ou resultados inexistentes.

# 3. REGRAS INEGOCIÁVEIS DE DESENVOLVIMENTO

## R1 — EVIDÊNCIA ANTES DE VALIDAR

Nenhuma funcionalidade pode ser considerada VALIDADA sem evidências reais armazenadas em:

`evidence/phaseN/`

As evidências podem incluir:

- Logs de execução.
- Capturas PNG.
- Arquivos WAV.
- Arquivos de vídeo.
- Saídas do FFprobe.
- Métricas de desempenho.
- Screenshots da interface.
- Relatórios automatizados.
- Resultados de testes de hardware.

Toda evidência deve indicar o teste executado e o ambiente utilizado.

## R2 — PROIBIDO SIMULAR FUNCIONALIDADES FALSAMENTE

É proibido utilizar:

- Valores aleatórios para simular dispositivos.
- Valores fixos para simular FPS.
- VU meters falsos.
- Bitrate inventado.
- Latência inventada.
- Indicadores falsos de conexão.
- Animações que representem processamento inexistente.
- Mock de hardware apresentado como hardware real.

Fontes sintéticas reais são permitidas para testes, desde que passem pelo pipeline audiovisual efetivo.

Exemplos:

- GStreamer videotestsrc.
- GStreamer audiotestsrc.
- FFmpeg lavfi testsrc2.
- FFmpeg lavfi smptebars.
- FFmpeg lavfi sine.

Toda fonte sintética deve ser identificada como `[SYNTH]` nos logs e nos indicadores de diagnóstico.

Um teste sintético não comprova funcionamento com hardware físico.

## R3 — BUILD PRIMEIRO

Após alterações relevantes, executar os testes aplicáveis.

Para Rust:

- `cargo fmt --check`
- `cargo check`
- `cargo test`
- `cargo clippy`

Para a aplicação web, executar os comandos de verificação e build definidos no projeto.

Não avançar com build quebrado.

Não desabilitar testes ou funcionalidades apenas para conseguir um build aparentemente aprovado.

## R4 — NUNCA AFIRMAR SEM EXECUTAR

Nunca afirmar que uma funcionalidade funciona sem executar o teste correspondente.

Quando não houver hardware, driver, serviço, permissão ou ambiente necessário, informar:

`IMPLEMENTADO-NÃO VALIDADO`

ou:

`BLOQUEADO`

Explicar exatamente o que falta para validar.

## R5 — GIT E CONTROLE DE ALTERAÇÕES

Antes de modificar arquivos:

1. Verificar o estado do Git.
2. Identificar alterações preexistentes.
3. Verificar os arquivos envolvidos.
4. Apresentar um plano curto.

Não sobrescrever alterações do usuário.

Não apagar arquivos, volumes, gravações, projetos ou configurações sem autorização explícita.

Commits e tags de fase somente após aprovação humana.

Não realizar push, publicação ou release sem autorização.

## R6 — LIMITE DE TENTATIVAS

Após três tentativas que falhem pela mesma causa raiz, interromper a tentativa de correção.

Apresentar:

- Logs.
- Hipótese da causa.
- Tentativas realizadas.
- Resultado de cada tentativa.
- Alternativas técnicas.

Não criar gambiarras para esconder problemas.

## R7 — PROCESSAMENTO EM TEMPO REAL

Os caminhos críticos de áudio e vídeo devem ser desenvolvidos com atenção rigorosa a latência, concorrência, alocação de memória e sincronismo.

São caminhos críticos:

- Captura.
- Decodificação.
- Composição.
- Mixagem.
- Processamento DSP.
- Callback de áudio.
- Encaminhamento de frames.

Callbacks de áudio não podem realizar:

- Alocação dinâmica.
- Mutex bloqueante.
- Operações de disco.
- Operações de rede.
- Logging síncrono.
- Chamadas demoradas de codec.

Utilizar buffers pré-alocados, filas apropriadas e mecanismos de sincronização adequados.

Toda otimização deve ser comprovada por medição.

## R8 — RUST E CÓDIGO UNSAFE

Priorizar Rust seguro.

Qualquer utilização de `unsafe` deve possuir:

- Justificativa técnica.
- Invariantes documentadas.
- Limites de responsabilidade.
- Testes apropriados.

FFI e SDKs externos devem permanecer isolados em adaptadores ou módulos específicos.

Não espalhar ponteiros crus e estruturas proprietárias pelo núcleo do sistema.

## R9 — BACKEND AUDIOVISUAL

GStreamer é o backend principal candidato.

FFmpeg deve ser utilizado como ferramenta complementar quando houver justificativa técnica.

Não duplicar funcionalidades sem necessidade.

Verificar:

- Versões.
- Plugins disponíveis.
- Licenças.
- Compatibilidade com Rust.
- Desempenho.
- Formatos.
- Codecs.
- Suporte de hardware.

Não presumir que um plugin está instalado apenas porque o GStreamer está instalado.

## R10 — CLEAN ROOM

O Ultra Stream deve possuir implementação independente.

Fontes permitidas:

- Documentação pública.
- Padrões abertos.
- Documentação de fabricantes.
- Especificações técnicas.
- Comportamento observável legitimamente acessível.
- Requisitos próprios.
- Analise REA
- Analise profunda do Vmix, pastas, subpastas e arquivos. 

É proibido:

- Copiar código.
- Copiar assets.
- Copiar ícones proprietários.


## R11 — LICENCIAMENTO

Toda dependência deve ser analisada antes da incorporação.

Registrar:

- Nome.
- Versão.
- Licença.
- Origem.
- Finalidade.
- Obrigações de distribuição.
- Restrições.
- Possibilidade de redistribuição.

Atenção especial a:

- GStreamer.
- FFmpeg.
- Plugins de codecs.
- NDI SDK.
- Blackmagic DeckLink SDK.
- AJA NTV2.
- Bibliotecas proprietárias.
- Codecs sujeitos a patentes.

Não redistribuir SDKs proprietários sem autorização.

Manter atualizados:

- `docs/LICENCIAMENTO.md`
- `THIRD-PARTY-LICENSES.md`

A licença proposta para o projeto deve ser revisada quanto à compatibilidade com as dependências antes da distribuição.

## R12 — MEMÓRIA ENTRE SESSÕES

No início de cada sessão, ler:

- `AGENTS.md`
- `docs/PROGRESS.md`
- `docs/ROADMAP.md`
- Documentos relacionados à tarefa.

Ao encerrar uma sessão, atualizar `docs/PROGRESS.md`.

Registrar:

- Fase atual.
- O que foi implementado.
- O que foi testado.
- O que falhou.
- Evidências.
- Decisões.
- Próximo passo exato.

Não apagar o histórico anterior.

## R13 — APROVAÇÃO HUMANA OBRIGATÓRIA

Ao concluir uma fase, interromper o desenvolvimento e apresentar o resultado.

Aguardar resposta:

`PASS`

ou:

`FAIL`

Não começar automaticamente a fase seguinte.

## R14 — PROTEÇÃO DO AMBIENTE DO USUÁRIO

Não modificar sem autorização:

- Configuração global do OpenCode.
- Provedor de IA.
- Modelo de IA.
- Endpoint.
- Credenciais.
- Tokens.
- Configurações do 9Router.
- MCPs globais.
- Variáveis de ambiente globais.
- Drivers.
- Firewall.
- Registro do Windows.
- Serviços do sistema.
- Dados e volumes Docker do usuário.

Preferir configurações locais do repositório.

Nunca incluir credenciais em logs ou commits.

## R15 — COMPATIBILIDADE

Projetos, presets, configurações, APIs e plugins precisam de versionamento.

Não quebrar projetos anteriores silenciosamente.

Toda alteração incompatível deve possuir migração documentada.

## R16 — FUNCIONALIDADE E INTERFACE

Uma funcionalidade destinada à operação deve possuir caminho de operação pela GUI desktop.

A mesma funcionalidade deve utilizar o modelo de comandos central do sistema.

A API web não deve implementar uma lógica paralela independente: deve enviar comandos ao mesmo núcleo utilizado pela aplicação desktop.

Exceções temporárias de prototipagem devem ser documentadas.

## R17 — TRANSPARÊNCIA SOBRE REA

Nunca afirmar que o REA foi consultado quando isso não aconteceu.

Nunca inventar uma ferramenta, endpoint ou integração.

Quando indisponível, informar claramente.

Os resultados do REA devem ser revisados e testados pelo coordenador.

# 4. ARQUITETURA DO SISTEMA

O Ultra Stream deve ser modular.

A arquitetura deve separar:

- Núcleo de domínio.
- Motor audiovisual.
- Captura.
- Decodificação.
- Composição.
- Áudio.
- Gravação.
- Streaming.
- Replay.
- Dispositivos.
- Automação.
- API.
- Persistência.
- Monitoramento.
- Plugins.
- Interface desktop.
- Interface web.

Nenhuma camada deve depender diretamente dos componentes visuais da interface.

## 4.1 Núcleo

O núcleo deve conter:

- ProjectState.
- CommandBus.
- Eventos.
- Configurações.
- Gerenciamento de estado.
- Regras de negócio.
- Interfaces dos subsistemas.

O estado central deve ser a fonte única de verdade.

## 4.2 Motor audiovisual

O processamento audiovisual deve acontecer localmente.

A aplicação web não deve ser responsável por executar o motor de vídeo.

O motor deve administrar:

- Inputs.
- Outputs.
- Pipelines.
- Buffers.
- Clocks.
- Timestamps.
- Sincronismo A/V.
- Composição.
- Mixagem.
- Gravação.
- Streaming.

## 4.3 Vídeo

Considerar:

- NV12.
- P010.
- BGRA.
- Outros formatos necessários.

Preservar metadados de cor, resolução, taxa de quadros, timestamps, interlace e HDR.

Evitar conversões desnecessárias.

A composição deve priorizar processamento GPU quando comprovadamente adequado.

## 4.4 Áudio

Considerar:

- 48 kHz como clock de referência inicial.
- Float32.
- Buffers circulares.
- Ring buffers SPSC quando adequados.
- Peak e RMS.
- Mixagem.
- Buses.
- Monitoramento.
- Compensação de drift.
- Sincronismo A/V.

A escolha de clock master e estratégias de sincronismo deve ser validada em protótipo.

## 4.5 Interface desktop

A aplicação desktop é a interface principal.

Deve permitir operação profissional de:

- Preview.
- Program.
- Inputs.
- Outputs.
- Transições.
- Overlays.
- Mixer.
- Gravação.
- Streaming.
- Replay.
- Multiview.
- Configurações.
- Automação.

## 4.6 Interface web

A aplicação web é complementar.

Deve permitir, conforme as fases forem implementadas:

- Controle de Preview e Program.
- Cut e transições.
- Controle de áudio.
- Controle de streaming.
- Gravação.
- Replay.
- Títulos.
- Overlays.
- Monitoramento.
- Configuração.
- Estado dos dispositivos.

A web UI deve se comunicar com a API local autenticada.

## 4.7 Plugins

A arquitetura deve permitir extensões futuras.

Avaliar:

- ABI C versionada.
- Isolamento de processos.
- Manifestos.
- Compatibilidade de versões.
- Permissões.
- Segurança.
- Falhas isoladas.

Não definir uma ABI definitiva sem protótipo.

# 5. ORGANIZAÇÃO DOS AGENTES DE IA

O OpenCode deve atuar como coordenador.

Agentes especializados sugeridos:

- Architect: arquitetura e decisões técnicas.
- Implementer: implementação.
- Reviewer: revisão independente.
- Tester: testes e evidências.
- Media Engine: pipelines audiovisuais.
- Audio Engine: áudio e DSP.
- Documentation: documentação e matriz de requisitos.

Cada agente deve receber tarefas delimitadas.

A delegação deve informar:

1. Objetivo.
2. Contexto.
3. Arquivos autorizados.
4. Arquivos que não podem ser alterados.
5. Interfaces que devem ser preservadas.
6. Critérios de aceitação.
7. Testes obrigatórios.
8. Formato do relatório.

Não permitir que dois agentes alterem simultaneamente os mesmos arquivos sem coordenação explícita.

O agente coordenador deve revisar todos os resultados.

# 6. MATRIZ DE FUNCIONALIDADES

Manter `docs/FEATURE_MATRIX.md`.

Cada funcionalidade deve possuir:

- ID.
- Categoria.
- Descrição.
- Fonte documental.
- URL.
- Edição de referência.
- Dependência externa.
- Licença.
- Fase.
- Estado.
- Critério de aceitação.
- Evidência.

Estados permitidos:

- NÃO INICIADO.
- EM IMPLEMENTAÇÃO.
- IMPLEMENTADO-NÃO VALIDADO.
- VALIDADO.
- BLOQUEADO.

A matriz deve possuir uma linha por funcionalidade e, quando aplicável, uma linha por operação de API.

Não declarar paridade com base em recursos apenas planejados.

Somente funcionalidades VALIDADO, com evidências, entram no percentual de paridade.

# 7. CICLO DE DESENVOLVIMENTO

Toda tarefa deve seguir:

1. Ler documentação.
2. Verificar Git.
3. Identificar a fase.
4. Apresentar plano.
5. Consultar REA, se disponível e pertinente.
6. Delegar tarefas quando necessário.
7. Implementar a menor alteração possível.
8. Compilar.
9. Executar testes.
10. Revisar o diff.
11. Registrar evidências.
12. Atualizar documentação.
13. Atualizar PROGRESS.md.
14. Apresentar relatório.

# 8. FASES DO PROJETO

Fase 0 — Governança, auditoria e especificação.

Fase 1 — Workspace Rust e GUI mínima.

Fase 2 — Modelo de mídia e fontes sintéticas reais.

Fase 3 — Dispositivos e captura.

Fase 4 — Compositor, Preview, Program e transições.

Fase 5 — Áudio e mixer.

Fase 6 — Inputs, layers e filtros.

Fase 7 — Gravação e streaming.

Fase 8 — Outputs profissionais e distribuição.

Fase 9 — Replay e gravação ISO.

Fase 10 — Gráficos, títulos e templates.

Fase 11 — API e controle web.

Fase 12 — Automação, MIDI, PTZ e integrações.

Fase 13 — Plugins e extensibilidade.

Fase 14 — Desempenho 4K e estabilidade.

Fase 15 — Testes UAT e distribuição.

Consultar `docs/ROADMAP.md` para os critérios específicos de cada fase.

# 9. TESTE DO USUÁRIO

Ao final de cada fase, apresentar:

=== TESTE DO USUÁRIO — FASE N ===

Pré-requisitos:
[Hardware, softwares e arquivos necessários]

Passos:
1. [Comando ou clique exato]
2. [Comando ou clique exato]

Resultado esperado:
1. [O que deve aparecer, tocar ou ser medido]
2. [Resultado esperado]

Evidências:
[Arquivos e caminhos necessários]

Responda com:

PASS

ou

FAIL — passo X + logs/screenshot.

Aguardar resposta antes de avançar.

# 10. RELATÓRIO DE FASE

Todo relatório deve informar:

1. Objetivo.
2. Problemas encontrados.
3. Causa raiz.
4. Arquivos criados.
5. Arquivos alterados.
6. Comandos executados.
7. Resultados de build.
8. Resultados de testes.
9. Evidências.
10. Funcionalidades validadas.
11. Funcionalidades implementadas não validadas.
12. Funcionalidades bloqueadas.
13. Riscos.
14. Uso ou indisponibilidade do REA.
15. Atualização do progresso.
16. Próximo passo exato.

# 11. INÍCIO DO PROJETO

O desenvolvimento deve começar pela Fase 0.

Não implementar funcionalidades audiovisuais antes de concluir:

- Auditoria do repositório.
- Inventário do ambiente.
- Matriz de funcionalidades.
- Revisão de licenciamento.
- Decisões arquiteturais iniciais.
- Planejamento técnico.

Ao concluir a Fase 0, interromper e aguardar aprovação humana.


## INTEGRAÇÃO RUST + C++

A arquitetura do Ultra Stream deve oferecer suporte nativo à integração entre Rust, C e C++. C++ não é uma tecnologia descartada nem está limitado a código legado. É uma linguagem complementar oficialmente prevista para a implementação de componentes audiovisuais, integração com hardware profissional e utilização de bibliotecas e SDKs nativos.

### 1. Divisão de responsabilidades

**Rust — núcleo principal**
- Orquestração geral do sistema.
- Gerenciamento de sessões, cenas, entradas e saídas.
- Gerenciamento de estado e ciclo de vida dos módulos.
- CommandBus e comunicação entre subsistemas.
- API e servidor de controle remoto.
- Interface desktop e integração com a interface web.
- Configuração, persistência, logs e monitoramento.
- Coordenação de recursos e gerenciamento de dispositivos.

**C++ — módulos nativos especializados**
- Integração com SDKs e APIs de fabricantes que disponibilizem interfaces C++.
- Captura e processamento audiovisual quando houver justificativa técnica.
- Componentes de baixa latência e alto desempenho quando benchmarks demonstrarem vantagem.
- Integração com bibliotecas nativas existentes.
- Adaptadores de hardware e funcionalidades que dependam de SDKs específicos.
- Componentes especializados de áudio, vídeo, GPU e processamento.

**C — interface de interoperabilidade**
- Definição da ABI pública entre os módulos.
- Contratos de comunicação independentes da linguagem de implementação.
- Compatibilidade entre versões de módulos e núcleo.

### 2. Fronteira de comunicação

A comunicação entre Rust e C++ deverá utilizar uma interface C versionada e documentada.

Não utilizar a ABI própria do C++ como contrato público entre módulos, devido aos problemas de compatibilidade entre compiladores, versões e toolchains.

A interface deverá contemplar:
- Identificação e versão da ABI.
- Identificação e versão do módulo.
- Estruturas de dados com tamanho e versão definidos.
- Handles opacos para objetos internos.
- Funções explícitas de criação, inicialização, execução e destruição.
- Tratamento padronizado de erros.
- Definição explícita de propriedade e liberação de memória.
- Contratos de concorrência e thread safety.
- Callbacks com convenção de chamada definida.
- Compatibilidade entre versões suportadas.

Não permitir que exceções C++ atravessem a fronteira da ABI. Exceções devem ser capturadas e convertidas em códigos de erro.

Não transmitir diretamente tipos complexos da STL, objetos C++ ou estruturas dependentes de compilador através da interface pública.

### 3. Estratégias de integração

O projeto deverá suportar duas modalidades de integração.

**Integração em processo (in-process)**

Para módulos confiáveis que necessitem de baixa latência e acesso direto aos recursos do processo principal.

Exemplos:
- Processadores especializados.
- Adaptadores de bibliotecas nativas.
- Componentes de composição e processamento.
- Módulos de áudio e vídeo.

**Integração fora do processo (out-of-process)**

Para SDKs, drivers, bibliotecas ou componentes que apresentem risco de instabilidade, incompatibilidade ou falha capaz de comprometer o núcleo.

A comunicação poderá utilizar:
- IPC.
- Memória compartilhada.
- Filas de mensagens.
- Handles de recursos.
- Compartilhamento de recursos GPU quando suportado.

A escolha deverá considerar latência, throughput, isolamento, segurança e complexidade operacional.

### 4. Hardware profissional

Criar uma camada Device Manager independente da implementação de cada fabricante.

Essa camada deverá permitir adaptadores específicos para:
- Blackmagic Design DeckLink.
- AJA Video Systems.
- Bluefish444.
- Dispositivos de áudio profissionais.
- Dispositivos de captura e saída de vídeo.
- Futuros fabricantes e interfaces.

Os adaptadores poderão ser escritos em Rust ou C++, conforme as APIs e SDKs disponíveis.

A integração deverá respeitar licenças, requisitos de distribuição, compatibilidade de drivers e limitações dos fabricantes.

Não implementar suporte fictício. Um dispositivo somente será considerado suportado após testes reais ou testes de conformidade claramente identificados.

### 5. Desempenho e caminhos críticos

Não estabelecer uma regra absoluta de que todo processamento audiovisual deva ser implementado em Rust ou C++.

A escolha deverá ser orientada por:
- Benchmarks reproduzíveis.
- Latência.
- Throughput.
- Uso de CPU e GPU.
- Alocações de memória.
- Estabilidade.
- Complexidade de manutenção.
- Disponibilidade de SDKs e bibliotecas.

Componentes críticos poderão ser implementados em C++ quando houver justificativa técnica comprovada.

A comunicação entre Rust e C++ deverá evitar cópias desnecessárias de buffers e conversões de formatos.

Callbacks de áudio em tempo real não poderão executar operações bloqueantes, alocações dinâmicas, I/O ou logging síncrono.

### 6. Compilação e integração de build

O projeto continuará utilizando Cargo como sistema principal de organização e compilação Rust.

Os módulos C e C++ deverão possuir seus próprios sistemas de build quando necessário, preferencialmente CMake para componentes C++.

A integração com o workspace Cargo deverá ser documentada e automatizada.

No Windows, a cadeia de compilação deverá contemplar MSVC e as dependências nativas necessárias.

A arquitetura deverá permitir futura compilação no Linux sem depender de caminhos, ferramentas ou bibliotecas exclusivas do Windows, exceto nos adaptadores específicos de plataforma.

Não introduzir dependências de compilação sem verificar licença, compatibilidade, manutenção e distribuição.

### 7. Segurança e estabilidade

Todo uso de FFI deverá ser documentado e revisado.

Em Rust:
- Isolar blocos unsafe.
- Documentar invariantes de segurança.
- Validar ponteiros, tamanhos e alinhamentos.
- Evitar uso de memória após sua liberação.
- Definir claramente quem é responsável pela propriedade dos buffers.

Em C++:
- Não permitir exceções atravessarem a ABI.
- Evitar comportamento indefinido.
- Controlar propriedade e ciclo de vida dos recursos.
- Validar os dados recebidos.
- Implementar tratamento de falhas.

### 8. Regra de decisão

Nenhum componente deverá ser escrito em C++ apenas por tradição ou preferência.

Nenhum componente deverá ser proibido de utilizar C++ apenas porque Rust é a linguagem principal.

A decisão deverá considerar a melhor solução técnica, o desempenho medido, os SDKs disponíveis, a segurança e a manutenção de longo prazo.

A arquitetura deverá manter Rust e C++ como tecnologias complementares e oficialmente suportadas pelo projeto.




FIM DAS INSTRUÇÕES DO PROJETO ULTRA STREAM.
