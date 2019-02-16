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
  --red: rgba(179, 0, 27, 1);
  --yellow: rgba(255, 231, 76, 1);
  --white: rgba(255, 255, 255, 1);
  --offwhite: rgba(255, 255, 255, 0.5);
  --slate: rgba(98, 131, 149, 1);
  --darkslate: rgb(8, 8, 8);
  background-color: var(--salmon);
}

body {
  color: var(--white);
  text-align: center;
  font-family: 'Josefin Sans', sans-serif;
  font-display: auto;
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
  margin-top: 1em;
}

.container h2 {
  font-size: 2em;
}

.icon {
  -webkit-filter: invert(1);
  filter: invert(1);
}

.icon.big {
  width: 144px;
  height: 144px;
  margin-bottom: 1em;
}

.card {
  padding: 2em 1em 1em 1em;
  box-shadow: inset 0 0 0 0.5em var(--white), inset 0 0 0 1em var(--slate);
  margin: 0 auto 1em auto;
  background: var(--white);
  color: var(--darkslate);
}
</style>
