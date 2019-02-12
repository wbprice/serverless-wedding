<template>
  <section class="card rsvp-card">
    <h2>{{ name }}</h2>
    <div class="field">
      <b-radio 
        :value="attending"
        native-value="true"
        @input="updateAttending">
        Yes! Can't Wait To Celebrate!
      </b-radio>
    </div>
    <div class="field">
      <b-radio
        :value="attending"
        native-value="false"
        @input="updateAttending">
        Sad To Say, We'll Miss Your Day
      </b-radio>
    </div>
  </section>
</template>

<script>
export default {
  name: 'RSVPCard',
  props: {
    name: {
      type: String,
      default: 'Name'
    },
    attending: {
      type: Boolean,
      default: false
    },
    id: {
      type: String,
      default: '1234'
    }
  },
  methods: {
    updateAttending(event) {
      const attending = event == 'true' ? true : false
      this.$store.commit('rsvp/toggle_attending', {
        id: this.id,
        attending
      })
    }
  }
}
</script>

<style>
.rsvp-card {
  padding: 2em 2em 2.5em 2em;
  max-width: 36em;
  box-shadow: inset 0 0 0 0.5em var(--white), inset 0 0 0 1em var(--slate);
  margin: 0 auto 1em auto;
}

.rsvp-card h2 {
  font-size: 2em;
  margin-bottom: 0.25em;
}
</style>
