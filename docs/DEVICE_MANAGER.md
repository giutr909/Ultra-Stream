
# Device Manager

## 1. Objetivo

Centralizar o gerenciamento de dispositivos de captura, saída e processamento audiovisual.

## 2. Responsabilidades

- Descoberta de dispositivos.
- Identificação do fabricante e modelo.
- Enumeração de capacidades.
- Inicialização e encerramento.
- Gerenciamento de entradas e saídas.
- Monitoramento de estado.
- Tratamento de desconexões.
- Recuperação de falhas.
- Disponibilização de capacidades ao núcleo.

## 3. Arquitetura

O Device Manager será implementado no núcleo Rust.

Cada fabricante poderá possuir um adaptador independente escrito em Rust ou C++.

Os adaptadores deverão respeitar contratos públicos e não poderão acessar diretamente estruturas internas privadas do núcleo.

## 4. Fabricantes previstos

- Blackmagic Design.
- AJA Video Systems.
- Bluefish444.

Outros fabricantes poderão ser adicionados posteriormente.

## 5. Requisitos dos adaptadores

Cada adaptador deverá declarar:

- Fabricante.
- Modelos suportados.
- Versões de SDK.
- Capacidades.
- Formatos de vídeo.
- Formatos de áudio.
- Resoluções e taxas de quadros.
- Recursos de sincronização.
- Requisitos de driver.
- Limitações conhecidas.

## 6. Validação

A validação deverá distinguir:

- Compilação do adaptador.
- Teste com dispositivo simulado.
- Teste com SDK real.
- Teste com hardware real.

Um nível não substitui o outro.

## 7. Falhas

A desconexão de um dispositivo deverá ser tratada sem comprometer o núcleo.

O Device Manager deverá registrar falhas e disponibilizar o estado do dispositivo para a interface desktop e para a API.
