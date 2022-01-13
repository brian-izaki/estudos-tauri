# Aplicação de vendas
Aplicação desktop/web desenvolvida com Vue e Tauri.

## Project setup
- ter o ambiente para a compilação do tauri.
  - [ambiente linux](https://tauri.studio/en/docs/getting-started/setup-linux) (tbm tem com o WSL2)
  - [ambiente windows](https://tauri.studio/en/docs/getting-started/setup-windows)
  - [ambiente macOS](https://tauri.studio/en/docs/getting-started/setup-macos)
- versão do node utilizada `16.13.0`
  ```shell
  # caso use o nvm, digite no terminal
  nvm use
  ```
- após configuração do ambiente, prossiga com a instalação usando npm
  ```shell
  npm install

  # executar aplicação desktop (tbm vai executar url pro navegador)
  npm run dev

  # executar aplicação somente no navegador
  npm run vue:dev
  ```
- caso queira fazer o build
  ```shell
  # somente aplicação desktop
  npm run build

  # somente aplicação web
  npm run vue:build
  ```
- outros comandos:
  ```shell
  # Verificar info sobre o projeto tauri
  npm run tauri:info

  # Lints and fixes files
  npm run lint
  ```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).

### References
- [tauri](https://tauri.studio/en/)
