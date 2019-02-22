const API_URL_ROOT =
  'https://1laad1x9sg.execute-api.us-east-1.amazonaws.com/dev'

export const state = () => ({
  request: {
    fetching: false,
    status_code: null,
    message: null
  },
  household: []
})

const editableKeys = [
  'attending',
  'dietary_restrictions',
  'dietary_restrictions_other',
  'food_preference'
]

function set_person_state(state, id, callback) {
  const index = state.household.findIndex(person => person.id == id)
  const person = state.household[index]
  return callback(person)
}

function get_patch_rsvp_request(axios, rsvp) {
  const payload = editableKeys.reduce((memo, item) => {
    if (rsvp.hasOwnProperty(item)) {
      memo[item] = rsvp[item]
    }
    return memo
  }, {})

  return axios.$patch(`${API_URL_ROOT}/rsvp/${rsvp.id}`, payload, {
    headers: {
      'Content-Type': 'application/json'
    }
  })
}

export const mutations = {
  fetch_household_request(state) {
    state.request.fetching = true
  },

  fetch_household_success(state, response) {
    state.request = {
      fetching: false,
      status_code: 200
    }
    state.household = response
  },

  fetch_household_failure(state) {
    state.request = {
      fetching: false,
      status_code: 500
    }
  },

  patch_household_request(state) {
    state.request = {
      fetching: true
    }
  },

  patch_household_success(state, response) {
    state.request = {
      fetching: false,
      status_code: 200
    }
    state.household = response
  },

  patch_household_failure(state, error) {
    state.request = {
      fetching: false,
      status_code: 500,
      message: 'Something went wrong'
    }
  },

  toggle_attending(state, { id, attending }) {
    set_person_state(state, id, person => {
      person.attending = attending
    })
  },

  set_dietary_restrictions(state, { id, diet }) {
    set_person_state(state, id, person => {
      if (diet.value != 'other') {
        person.dietary_restrictions_other = null
      }
      person.dietary_restrictions = diet.value
    })
  },

  set_dietary_restrictions_other(state, { id, value }) {
    set_person_state(state, id, person => {
      person.dietary_restrictions_other = value
    })
  },

  set_food_preference(state, { id, value }) {
    set_person_state(state, id, person => {
      person.food_preference = value
    })
  }
}

export const actions = {
  fetch_household({ commit }, householdId) {
    commit('fetch_household_request')
    return this.$axios
      .$get(`${API_URL_ROOT}/household/${householdId}`)
      .then(response => {
        commit('fetch_household_success', response)
      })
      .catch(error => {
        commit('fetch_household_failure', error)
        throw error
      })
  },

  patch_household({ commit }, household) {
    commit('patch_household_request')
    const requests = household.map(rsvp =>
      get_patch_rsvp_request(this.$axios, rsvp)
    )
    Promise.all(requests)
      .then(responses => {
        commit('patch_household_success', responses)
      })
      .catch(error => {
        commit('patch_household_failure', error)
      })
  }
}
