<template>
  <section class="rsvp">
    <h1>RSVP</h1>
    <p>{{ $route.params.household_id }}</p>

    <div
      v-for="person in household"
      :key="person.id">
      <RSVPCard
        :name="person.name"
        :id="person.id"
        :attending="person.attending" />
    </div>

    <div class="field">
      <button
        class="button is-primary is-large">
        Send</button>
    </div>
  </section>
</template>

<script>
import RSVPCard from './../../components/organisms/rsvp-card.vue'

export default {
  async asyncData({ store, params }) {
    const householdId = params.household_id
    await store.dispatch('rsvp/fetch_household', householdId)
  },
  components: {
    RSVPCard
  },
  computed: {
    household() {
      return this.$store.state.rsvp.household
    }
  }
}
</script>

<style>
</style>
