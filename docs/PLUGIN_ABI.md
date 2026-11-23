
# Especificação inicial da Plugin ABI

## 1. Objetivo

Definir os requisitos mínimos da interface pública dos plugins do Ultra Stream.

Esta especificação é preliminar e não representa uma ABI de produção já implementada.

## 2. Requisitos

Cada plugin deverá disponibilizar informações sobre:

- Identificador.
- Nome.
- Versão.
- Versão mínima e máxima da ABI suportada.
- Plataforma.
- Arquitetura.
- Dependências.
- Capacidades disponibilizadas.

## 3. Ciclo de vida

O contrato deverá prever:

1. Descoberta.
2. Verificação de compatibilidade.
3. Criação.
4. Inicialização.
5. Execução.
6. Solicitação de encerramento.
7. Destruição.

## 4. Segurança

A ABI deverá utilizar:

- Tipos C.
- Handles opacos.
- Códigos de erro.
- Estruturas versionadas.
- Propriedade explícita de recursos.

Não permitir exceções C++ atravessando a interface.

## 5. Compatibilidade

A incompatibilidade de ABI deverá impedir o carregamento do módulo.

A versão do plugin e a versão da ABI deverão ser verificadas separadamente.

## 6. Evolução

Alterações incompatíveis deverão gerar nova versão principal da ABI.

Alterações compatíveis deverão preservar os contratos anteriores.

## 7. Pendências

- Definir funções definitivas.
- Definir convenção de chamada.
- Definir regras de alocação e liberação.
- Definir callbacks.
- Definir contratos de concorrência.
- Definir mecanismo de carregamento.
- Definir política de compatibilidade.
