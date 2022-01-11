# Aplicação de vendas
Aplicação desktop/web desenvolvida com Vue e Tauri.

## Project setup
- ter o ambiente para a compilação do tauri.
  - [ambiente linux](https://tauri.studio/en/docs/getting-started/setup-linux) (tbm tem com o WSL2)
  - [ambiente windows](https://tauri.studio/en/docs/getting-started/setup-windows)
  - [ambiente macOS](https://tauri.studio/en/docs/getting-started/setup-macos)
- após configuração do ambiente, prossiga com a instalação usando npm
  ```shell
  npm install

  # executar aplicação somente no navegador
  npm run dev

  # executar aplicação desktop (tbm vai executar url pro navegador)
  npm run tauri:dev
  ```
- caso queira fazer o build
  ```shell
  # somente aplicação web
  npm run build

  # somente aplicação desktop
  npm run tauri:build
  ```

### Lints and fixes files
```shell
npm run lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).

### References
- [tauri](https://tauri.studio/en/)
