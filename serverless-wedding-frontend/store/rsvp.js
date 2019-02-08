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

  toggle_attending(uuid, attending) {
    debugger
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
  }
}
