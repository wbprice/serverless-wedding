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

function set_attending(state, { id, attending }) {
  const index = state.household.findIndex(function(person) {
    return person.id == id
  })
  state.household[index].attending = attending
}

function get_patch_rsvp_request(axios, id, attending) {
  return axios.$patch(
    `${API_URL_ROOT}/rsvp/${id}`,
    {
      attending: attending
    },
    {
      headers: {
        'Content-Type': 'application/json'
      }
    }
  )
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

  patch_rsvp_request(state, { id, attending }) {
    state.request = {
      fetching: true
    }
    set_attending(state, { id, attending })
  },

  patch_rsvp_success(state, response) {
    state.request = {
      fetching: false,
      status_code: 200
    }
    const { id, attending } = response
    set_attending(state, { id, attending })
  },

  patch_rsvp_failure(state, error) {
    state.request = {
      fetching: true,
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
    set_attending(state, { id, attending })
  }
}

export const actions = {
  fetch_household({ commit }, householdId) {
    commit('fetch_household_request')
    this.$axios
      .$get(`${API_URL_ROOT}/household/${householdId}`)
      .then(response => {
        commit('fetch_household_success', response)
      })
      .catch(error => {
        commit('fetch_household_failure', error)
      })
  },

  patch_rsvp({ commit }, { id, attending }) {
    commit('patch_rsvp_request', { id, attending })
    this.$axios
      .$patch(
        `${API_URL_ROOT}/rsvp/${id}`,
        {
          attending: attending
        },
        {
          headers: {
            'Content-Type': 'application/json'
          }
        }
      )
      .then(response => {
        commit('patch_rsvp_success', response)
      })
      .catch(error => {
        commit('patch_rsvp_failure', error)
      })
  },

  patch_household({ commit }, household) {
    commit('patch_household_request')
    const requests = household.map(rsvp =>
      get_patch_rsvp_request(this.$axios, rsvp.id, rsvp.attending)
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
