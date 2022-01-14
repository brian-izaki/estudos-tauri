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

## Anotações

<details>
  <summary>Estrutura do projeto</summary>

### **Estrutura do projeto**

- A aplicação tem 2 lados. _Front (HTML, CSS e JS)_ e o _Back (Rust)_
- `src`: fica o Front

  - nele é onde fica a aplicação Vue.js com os componentes de layout.

- `src-tauri`: fica o Back
  - diretório `src`: é onde os códigos em Rust ficam. Eles são responsáveis por se comunicar com o Sistema Operacional.
  - diretório `icons`: icones a serem usados pela aplicação desktop.
  - diretório `target` (gerado após primeira compilação): é onde o build pra aplicação desktop fica.
  - `tauri.conf.json`: arquivo responsável pela configuração do tauri
  - `Cargo.toml`: arquivo de gerenciamento de dependencias do código em rust. Semelhante ao _npm_
  - `rustfmt.toml`: arquivo para padronização da formatção do código em rust.

---

</details>

<details>
  <summary>Interação entre JS e Rust pelo tauri</summary>

- no back deve utilizar `invoke_handler` do tauri para permitir que a função em rust seja visível para o JavaScript. veja: [main.rs](src-tauri/src/main.rs#L12)
  ```rust
  // main.rs
  .invoke_handler(tauri::generate_handler![
      cmd::message_in_terminal
    ])
  ```
- no front devemos utilizar o `invoke` com o nome do método em string. veja: [Home.vue](src/views/Home.vue#L62)
  ```javascript
  invoke('message_in_terminal');
  ```
  - também podemos passar argumentos dentro do invoke caso o método no rust tenha paramentros

---

</details>

<br />

## References

- [tauri](https://tauri.studio/en/)
