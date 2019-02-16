<template>
  <section class="rsvp container">
    <h1>RSVP</h1>

    <RSVPCard
      v-for="person in household"
      :key="person.id"
      :name="person.name"
      :id="person.id"
      :attending="person.attending" />

    <div class="field">
      <button
        class="button is-primary is-large"
        @click="update_household">
        Send</button>
    </div>

  </section>
</template>

<script>
import RSVPCard from './../../../components/organisms/rsvp-card.vue'

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
  },
  methods: {
    update_household() {
      // Update the database
      this.$store.dispatch(
        'rsvp/patch_household',
        this.$store.state.rsvp.household
      )
      // Switch to the complete route
      this.$router.push({
        path: `/rsvp/${this.$route.params.household_id}/complete`
      })
    }
  }
}
</script>

<style>
</style>
