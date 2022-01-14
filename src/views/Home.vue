<template>
  <main>
    <section class="card-container">
      <router-link class="card pointer" to="/cart">
        <div>
          <p>Realizar compra</p>
        </div>
      </router-link>

      <div class="card">
        Info do sistema operacional:
        <br />
        {{ state.os }}
      </div>

      <div class="card pointer" @click="showMsgInTerminal">
        <p>Deve mandar mensagem para o terminal</p>
      </div>

      <div class="card">
        <p>Deve mandar o texto do input para o terminal</p>

        <input type="text" v-model="state.textoCustomizado" class="input" />
        <button type="button" @click="sendMsg2Terminal">Enviar</button>
      </div>

      <div class="card pointer" @click="openNewWindow">
        <p>Abre outra janela com título diferente</p>
      </div>
    </section>
  </main>
</template>

<script lang="ts">
import { defineComponent, onMounted, reactive } from "vue";
import { platform } from "@tauri-apps/api/os";
import { appWindow, WebviewWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  name: "Home",
  setup() {
    const state = reactive({
      os: "",
      textoTerminal: "Hello World",
      textoCustomizado: "",
    });

    onMounted(async () => {
      state.os = await platform();
    });

    appWindow.emit("event", "teste emit");

    const openNewWindow = () => {
      const webview = new WebviewWindow("window");
      webview.setTitle("Second");
      webview.emit("event");
    };

    const showMsgInTerminal = () => {
      invoke("message_in_terminal");
    };

    const sendMsg2Terminal = () => {
      invoke("send_message_to_terminal", { msg: state.textoCustomizado });
    };

    return { state, openNewWindow, showMsgInTerminal, sendMsg2Terminal };
  },
});
</script>

<style lang="scss" scoped>
main {
  margin: 20px 50px;
  display: flex;
  justify-content: space-between;
  gap: 30px;
}

.card-container {
  display: flex;
  flex-flow: row wrap;
  width: 80vw;
  gap: 20px;
}

.card {
  box-sizing: border-box;
  width: 200px;
  height: 200px;
  border-radius: 5px;
  background-color: #812f33e1;
  transition: transform 0.3s;
  color: #f3feb0;
  padding: 20px;

  &:hover {
    transform: translate(0, -5px);
  }
}

.pointer {
  cursor: pointer;
}

.horas {
  width: 50px;
  height: 200px;
  background-color: #f3feb0;
  margin-bottom: 20px;
}

.list-promocao {
  width: 50px;
  height: 200px;
  background-color: #fea443;
}

button {
  cursor: pointer;
}

.input {
  box-sizing: content-box;
  width: 100%;
  margin-bottom: 10px;
}
</style>
