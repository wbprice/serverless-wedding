const API_URL_ROOT = 

export const state = () => ({
  request: {
    fetching: false,
    status_code: null,
    message: null
  }
})

export const mutations = {
    fetch_household_request(state, {}) {

    },

    fetch_household_success(state, ) {

    },

    fetch_household_failure(state, ) {

    }
}

export const actions = {
    fetch_household({commit}) {
        commit('fetch_household_request');
        this.$axios.$get('')
    }
}