export const state = () => ({
  request: {
    fetching: false,
    status_code: null,
    message: null
  }
})

export const mutations = {
  add(state, text) {
    state.list.push({
      text: text,
      done: false
    })
  },

  remove(state, { todo }) {
    state.list.splice(state.list.indexOf(todo), 1)
  },

  toggle(state, todo) {
    todo.done = !todo.done
  }
}
