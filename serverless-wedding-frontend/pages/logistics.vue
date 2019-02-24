<template>
  <div class="our-story container">
    <h1>Logistics</h1>

    <Card>
      <h2>Air Travel</h2>
      <p>The closest airport is Denver International Airport (DEN).
      The last time we flew to Denver we used <a href="https://www.flyfrontier.com/">Frontier Airlines</a>.</p>
    </Card>

    <Card>
      <h2>Location</h2>
      <h3>Estes Park, CO</h3>
      <p>There are several ways to get to Estes Park:</p>
      <h4>Rent a Car</h4>
      <p>Boulder, CO is a a good halfway point to grab food and do other shopping.</p>
      <ul>
        <li><a href="https://priceline.com">Priceline.com</a></li>
        <li><a href="https://kayak.com">Kayak.com</a></li>
        <li><a href="https://carrentals.com">CarRentals.com</a></li>
      </ul>

      <h4>Ride Share</h4>
      <ul>
        <li><a href="https://www.lyft.com/rider/airports/den">Lyft</a></li>
        <li><a href="https://www.uber.com/global/en/airports/den/">Uber</a></li>
      </ul>

      <h4>Shuttle</h4>
      <ul>
        <li><a href="https://estesparkshuttle.com">Estes Park Shuttle</a></li>
        <li><a href="https://denversairporttransportation.com">denversairporttransportation.com</a></li>
        <li><a href="https://towncartoairportdenver.com">towncartoairportdenver.com</a></li>
      </ul>

    </Card>

    <Card>
      <h2>Rehearsal Afternoon</h2>
      <p>TBD</p>
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
      <h2>Food</h2>
      <p>Chef Grant from GB Culinary will prepare a four course Farm-to-table menu</p>
    </Card>

    <Card>
      <h2>Entertainment</h2>
      <p>Bring a boardgame to play!</p>
      <p>We're planning to have a billiards table and a hot tub, so bring a bathing suit!</p>
    </Card>

    <Card>
      <h2>Lodging</h2>
      <p>Estes is not a big town.  Anywhere you can find lodging is fine.</p>
      <h3>Suggestions</h3>
      <ul>
        <li>Historic Crag's Lodge</li>
        <li>The Ridgeline Hotel Estes Park</li>
        <li>The Stanley Hotel</li>
        <li>AirBnB / Homeaway</li>
      </ul>
    </Card>

    <Card>
      <h2>Local Activities</h2>
      
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

ul {
  margin-bottom: 16px;
}
</style>
