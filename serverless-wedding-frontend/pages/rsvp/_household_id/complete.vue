<template>
  <section class="rsvp-complete container">
    <h1>RSVP</h1>

    <Card v-if="going">
      <img
        alt="A paper plane being thrown"
        class="big icon"
        src="/paper-plane.png">
      <h2>We'll see you there!</h2>
      <p>Check the logistics page for more information!</p>
      <p>(P.S. bring boardgames)</p>
      <p>Look out for an invite on Facebook for further interactions and updates!</p>

      <nuxt-link
        class="button primary"
        to="/logistics">Logistics</nuxt-link>
    </Card>

    <Card v-else>
      <img
        alt="A present"
        class="big icon"
        src="/money.png">
      <h2>Sorry you can't make it!</h2>
      <p>There's always the registry!</p>
      <nuxt-link
        class="button primary"
        to="/registry">Registry</nuxt-link>
    </Card>
  </section>
</template>

<script>
import Card from './../../../components/molecules/card.vue'

export default {
  components: {
    Card
  },
  computed: {
    going() {
      return this.$store.state.rsvp.household.find(rsvp => rsvp.attending)
    }
  }
}
</script>

<style>
</style>
