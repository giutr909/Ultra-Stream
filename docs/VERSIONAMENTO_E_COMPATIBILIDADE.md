# VERSIONAMENTO E COMPATIBILIDADE

Utilizar versionamento SemVer quando houver releases públicas.

Versionar:

- Aplicativo.
- Projetos.
- Presets.
- API.
- Eventos.
- Comandos.
- Plugins.
- ABI.
- Configurações.

Projetos devem possuir schema_version.

Migrações devem ser testadas com arquivos de versões anteriores.

Nunca quebrar projetos antigos silenciosamente.

Plugins devem declarar compatibilidade com a versão do host.

Atualizações devem validar integridade e compatibilidade.

Manter matriz de compatibilidade de Windows, GPU, drivers, GStreamer, plugins e SDKs.
