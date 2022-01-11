<template>
  <main>
    <section class="card-container">
      <router-link class="card" to="/cart">
        <div>
          <p>Realizar compra</p>
        </div>
      </router-link>
      <div class="card"></div>
      <div class="card"></div>
    </section>

    <section>
      <div class="horas"></div>
      <div class="list-promocao"></div>
    </section>
  </main>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "Home",
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
  min-width: 400px;
  height: 200px;
  background-color: #812f33e1;
  transition: transform 0.3s;
  cursor: pointer;
  color: #f3feb0;

  &:hover {
    transform: translate(0, -5px);
  }
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
</style>
