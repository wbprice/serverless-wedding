<template>
  <section class="title container">
    <section class="rings">
      <div class="ring his" />
      <div class="ring hers" />
    </section>

    <h1>
      <span class="underline">Blaine</span>
      <span class="body-type"> is getting married to </span>
      <span class="underline">Ling Ling</span>
      <span class="date">July 6<sup>th</sup>, 2019</span>
    </h1>

    <section class="location body-type">
      <p>Somewhere around Denver</p>
    </section> 
  </section>
</template>

<script>
export default {}
</script>

<style>
.container {
  max-width: 960px;
}

.title {
  padding-top: 1em;
  font-size: 5em;
  line-height: 1.25em;
  color: var(--white);
}

.rings {
  display: flex;
  justify-content: center;
  margin-bottom: 0.5em;
}

.ring {
  width: 1em;
  height: 1em;
  border: 0.1em solid var(--darkslate);
  box-shadow: inset 0 0 0em 0.03em var(--white);
  border-radius: 100%;
}

.ring.his {
  margin-right: -0.25em;
  margin-bottom: -0.25em;
}

.ring.hers {
  margin-left: -0.25em;
  margin-top: -0.25em;
}

.date {
  margin-top: 0.25em;
  color: var(--darkslate);
  display: block;
}

.location {
  margin-top: 1em;
  font-size: 2rem;
}

sup {
  font-size: 0.5em;
}
</style>
