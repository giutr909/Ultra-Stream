# LICENCIAMENTO

O Ultra Stream tem como objetivo ser gratuito e de código aberto.

A licença proposta inicialmente é GPL-3.0-or-later, sujeita a revisão de compatibilidade antes da distribuição.

Toda dependência deve ser analisada individualmente.

## Componentes que exigem atenção

- GStreamer.
- FFmpeg.
- Plugins de codecs.
- wgpu.
- CPAL.
- Bibliotecas Rust.
- React.
- DeckLink SDK.
- AJA NTV2.
- NDI SDK.
- Codecs sujeitos a patentes.

## Processo

1. Identificar versão.
2. Identificar licença.
3. Consultar fonte oficial.
4. Verificar redistribuição.
5. Registrar obrigações.
6. Atualizar THIRD-PARTY-LICENSES.md.
7. Bloquear dependências incompatíveis até decisão.

Não redistribuir SDK proprietário sem autorização.

Não presumir que software gratuito significa ausência de royalties ou restrições.



# Licenciamento e dependências nativas

## 1. Linguagens

O projeto poderá utilizar componentes Rust, C e C++.

A linguagem de implementação não determina, por si só, a licença do componente. Cada biblioteca, SDK e dependência deverá ser avaliada individualmente.

## 2. Bibliotecas e SDKs

Antes de incorporar qualquer dependência, verificar:

- Licença.
- Direitos de redistribuição.
- Restrições comerciais.
- Obrigações de atribuição.
- Disponibilidade de código-fonte.
- Restrições de distribuição de binários.
- Compatibilidade com a licença do Ultra Stream.
- Dependências transitivas.

## 3. SDKs de fabricantes

SDKs de hardware profissional deverão possuir avaliação específica.

A utilização de SDK não implica automaticamente autorização para redistribuir seus arquivos ou incorporá-los ao instalador.

Documentar requisitos de instalação, licenças, versões e restrições.

## 4. Separação de dependências

Manter inventários separados para:

- Dependências Rust.
- Bibliotecas C.
- Bibliotecas C++.
- SDKs de fabricantes.
- Bibliotecas de runtime.
- Ferramentas de compilação.

## 5. Distribuição

A distribuição deverá considerar os requisitos de todas as dependências nativas e seus runtimes.

Nenhum componente será incorporado ao pacote de distribuição sem avaliação de sua licença e das obrigações aplicáveis.
