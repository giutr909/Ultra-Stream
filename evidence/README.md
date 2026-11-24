# Evidências — Ultra Stream

Este diretório armazena as evidências reais de validação de cada fase, conforme a regra **R1 — EVIDÊNCIA ANTES DE VALIDAR** do `AGENTS.md`.

## Regras

1. Cada fase possui seu próprio subdiretório: `phase0/`, `phase1/`, `phase2/`, ...
2. Toda evidência deve indicar o teste executado e o ambiente utilizado.
3. É proibido criar evidências fictícias, simuladas ou preenchidas manualmente sem execução real.
4. Um subdiretório vazio significa que **nenhuma evidência foi coletada ainda** — não significa validação.

## Estado atual

| Diretório | Estado | Observação |
|---|---|---|
| `phase0/` | vazio | Fase 0 (governança e auditoria) — sem evidências coletadas neste diretório |
| `phase1/` | vazio | Fase 1 (auditoria de ambiente) — resultados registrados em `docs/FASE_1_RELATORIO.md` e `docs/FASE_1_CORRECOES_RELATORIO.md` |
| `phase2/` | vazio | Fase 2 não iniciada |
| `phase3/` | vazio | Fase 3 não iniciada |

Os arquivos `.gitkeep` existem apenas para preservar a estrutura de diretórios no Git e não constituem evidência.
