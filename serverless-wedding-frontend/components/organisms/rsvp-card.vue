<template>
  <section class="card rsvp">
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
.rsvp.card {
  max-width: 36em;
}

.rsvp-card h2 {
  font-size: 2em;
  margin-bottom: 0.25em;
}
</style>
