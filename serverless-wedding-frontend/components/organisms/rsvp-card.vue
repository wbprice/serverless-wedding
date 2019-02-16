<template>
  <section class="card rsvp">
    <h2>{{ name }}</h2>
    <div class="field">
      <p-radio
        id="true"
        :checked="attending"
        :name="name"
        class="p-default p-round"
        color="info"
        @change="updateAttending">
        Yes! Can't Wait To Celebrate!
      </p-radio>
    </div>
    <div class="field">
      <p-radio
        id="false"
        :checked="!attending"
        :name="name"
        class="p-default p-round"
        color="danger"
        @change="updateAttending">
        Sad To Say, We'll Miss Your Day
      </p-radio>
    </div>
  </section>
</template>

<script>
import PrettyRadio from 'pretty-checkbox-vue/radio'

export default {
  name: 'RSVPCard',
  components: {
    'p-radio': PrettyRadio
  },
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
      const attending = !this.attending
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

.field {
  margin-bottom: 1em;
}
</style>
