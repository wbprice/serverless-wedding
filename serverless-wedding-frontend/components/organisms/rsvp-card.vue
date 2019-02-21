<template>
  <Card>
    <h2>{{ name }}</h2>

    <b-field label="Can you make it?" />
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

    <div class="field-group">
      <b-field label="Dietary Restrictions">
        <b-select
          :selected="dietary_restrictions"
          :value="dietary_restrictions"
          @input="updateDietaryRestrictions" >
          <option
            v-for="diet in dietaryRestrictions"
            :key="diet.value"
            :value="diet.value">
            {{ diet.label }}
          </option>
        </b-select>
      </b-field>

      <b-field label="If other, please add details below">
        <b-input
          :disabled="otherDisabled"
          :value="dietary_restrictions_other"
          @input="updateDietaryRestrictionsOther" />
      </b-field>
    </div>

    <b-field label="How many kids are you bringing?">
      <b-input
        :value="children_count"
        type="number"
        min="0"
        max="10"
        @input="updateChildrenCount" />
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
    },
    dietary_restrictions_other: {
      type: String,
      default: null
    },
    children_count: {
      type: Number,
      default: 0
    }
  },
  data() {
    return {
      dietaryRestrictions: [
        {
          value: 'vegetarian',
          label: 'Only vegetables, no animal products please.'
        },
        {
          value: 'gluten-free',
          label: "I don't do bread"
        },
        {
          value: 'none',
          label: 'I can eat anything'
        },
        {
          value: 'pescatarian',
          label: 'I like eating vegetables and fish'
        },
        {
          value: 'other',
          label: 'Other (Please add detail below)'
        }
      ]
    }
  },
  computed: {
    otherDisabled() {
      return this.dietary_restrictions != 'other'
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
    updateDietaryRestrictions(event) {
      const diet = this.dietaryRestrictions.find(diet => diet.value == event)
      this.$store.commit('rsvp/set_dietary_restrictions', {
        id: this.id,
        diet
      })
    },
    updateDietaryRestrictionsOther(event) {
      const value = event
      this.$store.commit('rsvp/set_dietary_restrictions_other', {
        id: this.id,
        value
      })
    },
    updateChildrenCount(event) {
      const value = event
      this.$store.commit('rsvp/set_children_count', {
        id: this.id,
        value
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
  max-width: 360px;
  margin-left: auto;
  margin-right: auto;
}

.field-group {
  padding: 0.5em;
  max-width: 480px;
  margin: 0 auto 1em auto;
  border: 2px solid var(--salmon);
}

.field .control {
  text-align: center;
}
</style>
