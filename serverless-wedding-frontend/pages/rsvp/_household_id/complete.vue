<template>
  <section class="rsvp-complete container">
    <h1>RSVP Completed</h1>

    <div 
      v-if="going"
      class="going">
      <h2>We'll see you there!</h2>
    </div>

    <div 
      v-else
      class="not-going">
      <h2>Sorry you can't make it!</h2>
      <p>There's always the registry!</p>
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
