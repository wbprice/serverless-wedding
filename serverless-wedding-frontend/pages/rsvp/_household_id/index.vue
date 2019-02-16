<template>
  <section class="rsvp container">
    <h1>RSVP</h1>

    <section>
      <RSVPCard
        v-for="person in household"
        :key="person.id"
        :name="person.name"
        :id="person.id"
        :attending="person.attending"
      />

      <div
        v-if="householdId && household.length"
        class="field">
        <button
          class="button is-primary is-large"
          @click="update_household">Send</button>
      </div>
    </section>

    <section v-if="!householdId">
      <img
        alt="An email being sent"
        class="big icon" 
        src="~/assets/message.png">

      <div class="card">
        <h2>Your Invite's In The Mail!</h2>
        <p>We sent your invite by email.  Check your inbox!</p>
      </div>
    </section>
  </section>
</template>

<script>
import RSVPCard from './../../../components/organisms/rsvp-card.vue'

export default {
  components: {
    RSVPCard
  },
  computed: {
    householdId() {
      return this.$route.params.household_id
    },
    household() {
      return this.$store.state.rsvp.household
    }
  },
  mounted() {
    const householdId = this.$route.params.household_id
    if (householdId) {
      this.$store.dispatch('rsvp/fetch_household', householdId).catch(() => {
        // Switch to the error route
        this.$router.push({
          path: `/rsvp/error`
        })
      })
    }
  },
  methods: {
    update_household() {
      // Update the database
      this.$store
        .dispatch('rsvp/patch_household', this.$store.state.rsvp.household)
        .then(() => {
          // Switch to the complete route
          this.$router.push({
            path: `/rsvp/${this.$route.params.household_id}/complete`
          })
        })
    }
  }
}
</script>

<style>
</style>
