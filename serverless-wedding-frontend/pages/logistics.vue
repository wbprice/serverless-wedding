<template>
  <div class="our-story container">
    <h1>Logistics</h1>

    <Card>
      <h2>Air Travel</h2>
      <p>The closest airport is Denver International Airport (DEN).
      The last time we flew to Denver we used <a href="https://www.flyfrontier.com/">Frontier Airlines</a>.</p>
    </Card>

    <Card>
      <h2>Rehearsal Dinner</h2>
      <p>TBD</p>
    </Card>

    <Card>
      <h2>Ceremony</h2>
      <p>The ceremony and reception will be held in Estes Park, CO. There are several ways to get to Estes:</p>
      <ul>
        <li>Rent a car</li>
        <li>Uber</li>
        <li>Airport Shuttle</li>
      </ul>
    </Card>

    <Card>
      <h2>Entertainment</h2>
      <p>Bring a boardgame to play!</p>
    </Card>

  </div>
</template>

<script>
import Card from './../components/molecules/card.vue'

export default {
  components: {
    Card
  }
}
</script>

<style>
</style>
