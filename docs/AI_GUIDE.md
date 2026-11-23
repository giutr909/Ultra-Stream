
# Guia de contexto para agentes de IA — Ultra Stream

## Identidade do projeto

Ultra Stream é um software de produção audiovisual ao vivo, com GUI desktop para Windows e aplicação web complementar para controle e monitoramento.

O motor audiovisual será executado localmente.

O projeto busca evolução funcional em direção à categoria do vMix 4K Pro, por implementação independente e respeitando licenças e direitos de terceiros.

## Arquitetura

- Rust: linguagem principal.
- C++: linguagem complementar de primeira classe.
- C: interface de interoperabilidade e ABI versionada.
- GStreamer: backend audiovisual principal candidato.
- FFmpeg: tecnologia complementar.
- wgpu: candidato para processamento GPU.
- egui/eframe: candidato para GUI desktop.
- Axum e WebSocket: candidatos para API.
- React e TypeScript: candidatos para interface web.

Consulte docs/ARQUITETURA.md para a arquitetura completa.

## Regras fundamentais

- Não inventar funcionalidades implementadas.
- Não declarar testes executados sem executá-los.
- Não substituir decisões arquiteturais sem justificativa.
- Não descartar alterações existentes.
- Não modificar configurações globais do desenvolvedor.
- Não implementar funcionalidades fictícias para simular suporte real.
- Não copiar código ou identidade visual de produtos proprietários.
- Manter Rust e C++ como tecnologias oficialmente suportadas.
- Registrar decisões arquiteturais importantes.
- Atualizar docs/PROGRESS.md ao concluir cada etapa.

## Como iniciar qualquer tarefa

1. Ler AGENTS.md ou as instruções específicas da ferramenta.
2. Ler este documento.
3. Consultar docs/PROGRESS.md.
4. Consultar docs/ROADMAP.md.
5. Consultar os documentos técnicos relacionados à tarefa.
6. Inspecionar o código existente antes de modificar.
7. Apresentar o plano antes de alterações arquiteturais importantes.

## Fonte oficial das decisões

A documentação de docs/ representa as decisões e especificações do projeto.

Se encontrar contradições, não escolher arbitrariamente uma versão. Apresentar o conflito e solicitar resolução quando necessário.
