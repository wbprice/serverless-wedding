<template>
  <section class="rsvp-complete container">
    <h1>RSVP</h1>

    <div 
      v-if="going"
      class="card going">
      <img
        alt="A paper plane being thrown"
        class="big icon"
        src="paper-plane.png">
      <h2>We'll see you there!</h2>
      <p>Check the logistics page for more information!</p>
      <nuxt-link
        class="button primary"
        to="/registry">Logistics</nuxt-link>
    </div>

    <div 
      v-else
      class="card not-going">
      <img
        alt="A present"
        class="big icon"
        src="money.png">
      <h2>Sorry you can't make it!</h2>
      <p>There's always the registry!</p>
      <nuxt-link
        class="button primary"
        to="/registry">Registry</nuxt-link>
    </div>
  </section>
</template>

<script>
export default {
  computed: {
    going() {
      return this.$store.state.rsvp.household.find(rsvp => rsvp.attending)
    }
  }
}
</script>

<style>
</style>
