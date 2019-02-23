<template>
  <div class="our-story container">
    <h1>Logistics</h1>

    <Card>
      <h2>Air Travel</h2>
      <p>The closest airport is Denver International Airport (DEN).
      The last time we flew to Denver we used <a href="https://www.flyfrontier.com/">Frontier Airlines</a>.</p>
    </Card>

    <Card>
      <h2>Rehearsal Afternoon</h2>
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
      <h2>Venue</h2>
      <img
        class="venue"
        alt="A mountain lodge"
        src="/venue.jpg" >
      <p>The venue for the ceremony and reception will be a mountain home in Estes Park, CO</p>
      <ul>
        <li>We'll have beds for 5 couples, available on a first-come, first served basis</li>
        <li>The house doesn't have air conditioning</li>
        <li>There are bears in the area. Please secure all trash. You'll need to lock doors and windows each time you leave the property.</li>
        <li>No smoking</li>
        <li>We have one high chair</li>
      </ul>
    </Card>

    <Card>
      <h2>Entertainment</h2>
      <p>Bring a boardgame to play!</p>
      <p>We're planning to have a billiards table and a hot tub, so bring a bathing suit!</p>
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
.venue {
  max-width: 300px;
}
</style>
