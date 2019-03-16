<template>
  <section class="rsvp container">
    <h1>RSVP</h1>

    <section v-if="householdId && household.length">

      <header>
        <h2>We want to have you at our wedding!</h2>
        <p>Please reply by <em>April 15th, 2019</em></p>
      </header>

      <RSVPCard
        v-for="person in household"
        :key="person.id"
        :name="person.name"
        :id="person.id"
        :attending="person.attending"
        :dietary_restrictions="person.dietary_restrictions"
        :dietary_restrictions_other="person.dietary_restrictions_other"
        :food_preference="person.food_preference"
      />

      <button
        class="button primary"
        @click="update_household">Send</button>
    </section>

    <section v-if="!householdId">

      <div class="card">
        <img
          alt="An email being sent"
          class="big icon"
          src="/message.png">

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
          path: `/rsvp/${householdId}/error`
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
.rsvp header {
  margin-bottom: 2em;
}
</style>
