# WORKFLOW REA + IA + OPENCODE

## Papéis

Responsável humano:
- Define prioridades.
- Aprova decisões.
- Aprova mudanças estruturais.
- Aprova fases.

OpenCode:
- Coordena agentes.
- Analisa tarefas.
- Distribui trabalho.
- Integra alterações.
- Executa validações.
- Produz relatórios.

REA:
- Recurso especializado quando disponível.
- Deve ser consultado conforme sua capacidade real.
- Não presumir integração inexistente.

Agentes:
- Executam tarefas delimitadas.
- Não alteram escopo aprovado.
- Não realizam mudanças destrutivas sem autorização.

## Início de sessão

1. Ler AGENTS.md.
2. Ler PROGRESS.md.
3. Verificar Git.
4. Identificar fase.
5. Identificar pendências.
6. Verificar ferramentas disponíveis.
7. Planejar a tarefa.

## Delegação

Toda delegação deve definir objetivo, arquivos autorizados, interfaces, critérios de aceitação e testes.

Evitar dois agentes editando simultaneamente o mesmo arquivo.

O coordenador deve revisar todos os resultados.

## Proteção do ambiente

Não modificar configurações globais do OpenCode, modelos, provedores, endpoints, 9Router, credenciais ou MCPs sem autorização.

Preferir configurações locais do projeto.

## Encerramento

Registrar alterações, testes, evidências, falhas, decisões e próximo passo.

Ao final de cada fase, aguardar aprovação humana.
