<template>
  <section class="rsvp">
    <h1>RSVP</h1>
    <p>{{ $route.params.household_id }}</p>
    <div class="field">
      <b-switch 
        type="is-info"
        size="is-large">Will You Be There?</b-switch>
    </div>
    <div class="field">
      <button class="button is-primary is-large">Send</button>
    </div>
  </section>
</template>

<script>
export default {
  asyncData({ store, params }) {
    const householdId = params.household_id
    store.dispatch('fetch_household_request', householdId)
  }
}
</script>

<style>
</style>
