<template>
  <div>
    <nuxt/>
  </div>
</template>

<style>
:root {
  --salmon: #ef958d;
  --red: rgba(179, 0, 27, 1);
  --yellow: rgba(255, 231, 76, 1);
  --white: rgba(255, 255, 255, 1);
  --slate: rgba(98, 131, 149, 1);
  --darkslate: rgba(98, 131, 149, 1);
  background-color: var(--salmon);
}

body {
  color: var(--white);
  text-align: center;
  font-family: 'Josefin Sans', sans-serif;
  font-display: auto;
}

.body-type {
  font-family: 'Josefin Sans', sans-serif;
}

.underline {
  text-decoration: underline;
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
  margin-bottom: 0.5em;
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
}
</style>
