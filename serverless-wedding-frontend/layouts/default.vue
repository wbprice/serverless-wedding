<template>
  <div>
    <Menu />
    <nuxt/>
  </div>
</template>

<script>
import Menu from './../components/organisms/menu.vue'

export default {
  components: {
    Menu
  }
}
</script>


<style>
:root {
  --salmon: #db6156;
  --lightsalmon: #ecb0aa;
  --red: rgba(179, 0, 27, 1);
  --yellow: rgba(255, 231, 76, 1);
  --white: rgba(255, 255, 255, 1);
  --offwhite: rgba(255, 255, 255, 0.5);
  --lightslate: rgba(143, 189, 216, 1);
  --slate: rgba(98, 131, 149, 1);
  --darkslate: rgb(8, 8, 8);
  background-color: var(--salmon);
}

body {
  color: var(--white);
  text-align: center;
  font-family: 'Josefin Sans', sans-serif;
  font-display: auto;
  font-size: 16px;
}

h1,
h2,
h3,
h4,
h5 {
  font-family: 'Abril Fatface', Georgia, 'Times New Roman', Times, serif;
  font-display: auto;
}

.container {
  margin: 0 auto;
  max-width: 720px;
}

.container h1 {
  font-size: 3em;
  margin-top: 0.5em;
  margin-bottom: 0.5em;
}

.container h2 {
  font-size: 2em;
}

.icon.big {
  width: 144px;
  height: 144px;
}

.button.primary {
  background: var(--lightslate);
  border-color: var(--slate);
  color: var(--white);
}

.button.primary:hover {
  background: var(--slate);
}
</style>
