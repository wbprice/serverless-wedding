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

    <b-field label="What Do You Want To Eat?">
      <b-select
        :disabled="disabled"
        :selected="food_preference"
        :value="food_preference"
        @input="updateFoodPreference" >
        <option
          v-for="food in foodPreferences"
          :key="food.value"
          :value="food.value">
          {{ food.label }}
        </option>
      </b-select>
    </b-field>

    <div class="field-group">
      <b-field label="Dietary Restrictions">
        <b-select
          :disabled="disabled"
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
    food_preference: {
      type: String,
      default: '4course'
    }
  },
  data() {
    return {
      dietaryRestrictions: [
        {
          value: 'none',
          label: 'None - I can eat anything'
        },
        {
          value: 'pescatarian',
          label: 'Pescatarian - Vegetables & Fish'
        },
        {
          value: 'vegetarian',
          label: 'Vegetarian - No meat, please'
        },
        {
          value: 'vegan',
          label: 'Vegan - No Animal Products'
        },
        {
          value: 'gluten-free',
          label: 'Gluten-Free - No Bread'
        },
        {
          value: 'other',
          label: 'Other (Please specify)'
        }
      ],
      foodPreferences: [
        {
          value: '4course',
          label: 'Four-Course Meal'
        },
        {
          value: 'pizza',
          label: 'Pizza'
        }
      ]
    }
  },
  computed: {
    otherDisabled() {
      return this.dietary_restrictions != 'other'
    },
    disabled() {
      return this.attending == false
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
    updateFoodPreference(event) {
      const value = event
      this.$store.commit('rsvp/set_food_preference', {
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
