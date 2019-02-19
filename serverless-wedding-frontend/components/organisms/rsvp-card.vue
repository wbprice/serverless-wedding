<template>
  <Card>
    <h2>{{ name }}</h2>

    <div class="field">
      <b-radio 
        :name="id"
        :value="attending"
        native-value="true"
        @input="updateAttending">
        Yes! Can't Wait to Celebrate!
      </b-radio>
    </div>

    <div class="field">
      <b-radio
        :name="id"
        :value="attending"
        native-value="false"
        @input="updateAttending">
        Sorry to Say, We'll Miss Your Day
      </b-radio>
    </div>

    <b-field label="Dietary Restrictions">
      <b-select
        :selected="dietary_restrictions.label"
        :value="dietary_restrictions.label"
        placeholder="None"
        @input="updateDietaryRestriction" >
        <option
          v-for="diet in dietaryRestrictions"
          :key="diet.key"
          :value="diet.label">
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
    },
    dietary_restrictions: {
      type: String,
      default: 'none'
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
      const attending = event == 'true' ? true : false
      this.$store.commit('rsvp/toggle_attending', {
        id: this.id,
        attending
      })
    },
    updateDietaryRestriction(event) {
      const diet = this.dietaryRestrictions.find(diet => diet.label == event)
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

.field .control {
  text-align: center;
}
</style>
