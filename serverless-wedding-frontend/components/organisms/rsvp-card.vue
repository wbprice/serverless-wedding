<template>
  <Card>
    <h2>{{ name }}</h2>

    <div class="field">
      <b-radio 
        native-value="true">
        Yes! Can't Wait to Celebrate!
      </b-radio>
    </div>
    <div class="field">
      <b-radio 
        native-value="false">
        Sorry to Say, We'll Miss Your Day
      </b-radio>
    </div>

    <b-field label="Dietary Restrictions">
      <b-select 
        placeholder="None"
        @input="updateDietaryRestriction" >
        <option
          v-for="diet in dietaryRestrictions"
          :key="diet.key">
          {{ diet.label }}
        </option>
      </b-select>
    </b-field>
  </Card>
</template>

<script>
import Card from './../../components/molecules/card'

export default {
  name: 'RSVPCard',
  components: {
    Card
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
  data() {
    return {
      dietaryRestrictions: [
        {
          key: 'vegetarian',
          label: 'Only vegetables, no animal products please.'
        },
        {
          key: 'gluten-free',
          label: "I don't do bread"
        },
        {
          key: 'none',
          label: 'I can eat anything'
        },
        {
          key: 'pescatarian',
          label: 'I like eating vegetables and fish'
        }
      ]
    }
  },
  methods: {
    updateAttending(event) {
      const attending = !this.attending
      this.$store.commit('rsvp/toggle_attending', {
        id: this.id,
        attending
      })
    },
    updateDietaryRestriction(event) {
      const diet = this.dietaryRestrictions.find(diet => (diet.label = event))
      this.$store.commit('rsvp/set_dietary_restriction', {
        id: this.id,
        diet
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
