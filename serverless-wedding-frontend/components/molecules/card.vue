<template>
  <div class="card"> 
    <slot />
  </div>
</template>

<script>
export default {
  name: 'Card'
}
</script>

<style>
.card {
  padding: 2em 3em;
  box-shadow: inset 0 0 0 0.5em var(--white), inset 0 0 0 1em var(--lightslate);
  margin: 0 auto 1em auto;
  background: var(--white);
  color: var(--darkslate);
}

.card img {
  max-width: 100%;
}

.card .button {
  border-width: 2px;
  margin: 1em 0 0.5em 0;
}

.card h2 {
  margin-bottom: 0.25em;
}

.card p {
  margin: 1em 0;
}
</style>
