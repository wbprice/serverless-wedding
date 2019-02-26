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
        <li>
          <a href="https://estesparkshuttle.com">Estes Park Shuttle</a>
          <ul>
            <li>(970) 586-5151</li>
            <li>$45 one way or $85.00 per person for round trip</li>
            <li>Use our special coupon code to get 10% off!  Codeword: "blinginglove"</li>
          </ul>
        </li>
        <li>
          <a href="https://denversairporttransportation.com">Denvers Airport Transportation</a>
          <ul>
            <li>(720) 579-6135</li>
            <li>SUV 5 people $195</li>
            <li>14 passengers Mercedes Sprinters minibus $375</li>
          </ul>
        </li>
        <li>
          <a href="https://towncartoairportdenver.com">Town Car Airport Denver</a>
          <ul>
            <li>(303) 587-8882</li>
            <li>7 people $250</li>
          </ul>
        </li>
      </ul>

    </Card>

    <Card>
      <h2>Venue</h2>
      <img
        class="venue"
        alt="A mountain lodge"
        src="/venue.jpg" >
      <p>The venue for the ceremony and reception will be a mountain home in Estes Park, CO.</p>
      <ul>
        <li>If you party too hard on the night of the wedding, we'll have beds for 5 couples on a first-come, first served basis.</li>
        <li>The house doesn't have air conditioning.</li>
        <li>There are bears in the area. Please secure all trash. You'll need to lock doors and windows each time you leave the property or your car.</li>
        <li>No smoking.</li>
        <li>We have one high chair.</li>
      </ul>
    </Card>

    <Card>
      <h2>Rehearsal Afternoon</h2>
      <p>TBD</p>
    </Card>

    <Card>
      <h2>Food</h2>
      <p>Chef Grant from GB Culinary will prepare a four course farm-to-table menu.</p>
      <p>We'll order pizza for those who have simpler tastes.</p>
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
        <li>
          <a href="https://www.diamondresorts.com/destinations/property/The-Historic-Crags-Lodge">
            Historic Crag's Lodge
          </a>
        </li>
        <li>
          <a href="https://ridgelinehotel.com/">
            The Ridgeline Hotel Estes Park
          </a>
        </li>
        <li>
          <a href="https://www.stanleyhotel.com/">
            The Stanley Hotel
          </a>
        </li>
        <li>
          <a href="https://www.airbnb.com/a/Estes-Park--United-States">
            AirBnB
          </a>
        </li>
        <li>
          <a href="https://www.homeaway.com/d/128/estes-park">
            Homeaway
          </a>
        </li>
      </ul>
    </Card>

    <Card>
      <h2>Local Activities</h2>
      <p>Estes Park is known for it's natural beauty and outdoor attractions</p>
      <ul>
        <li><a href="https://www.visitestespark.com/events/community-all-levels-yoga/5316/">Free Yoga on 7/5/19 from 6-7 PM</a></li>
        <li>Rocky Mountain National Park</li>
        <li>Horseback Riding</li>
        <li>Visit <a href="https://www.tripadvisor.com/Tourism-g60945-Estes_Park_Colorado-Vacations.html">TripAdvisor</a> for other ideas</li>
        <li><a href="https://colorado.com/events">Colorado.com Events</a></li>
        <li><a href="https://estes-park.com/free-shuttle-buses/">Free shuttle around town and to parks</a></li>
      </ul>
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
