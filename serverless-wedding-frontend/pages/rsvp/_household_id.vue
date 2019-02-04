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
  async asyncData({ store, params }) {
    const householdId = params.household_id
    await store.dispatch('rsvp/fetch_household', householdId)
  }
}
</script>

<style>
</style>
