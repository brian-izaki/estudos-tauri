<template>
  <nav>
    <div class="menu-container">
      <p></p>
      <h1>{{ routeName }}</h1>
      <p>Bem vindo Brian!</p>
    </div>
  </nav>
</template>

<script>
import { defineComponent, ref, watch } from "vue";
import { useRoute } from "vue-router";
export default defineComponent({
  setup() {
    const route = useRoute();
    const routeName = ref();

    watch(
      () => route.name,
      (name) => (routeName.value = name)
    );

    return {
      routeName,
    };
  },
});
</script>

<style lang="scss" scoped>
nav {
  height: 10vh;
  background: #705e78;
}

.menu-container {
  height: 100%;
  display: flex;
  justify-content: space-between;
  align-items: center;
  list-style: none;
  color: #f3feb0;

  & p,
  & h1 {
    flex: 1;
  }
}
</style>
