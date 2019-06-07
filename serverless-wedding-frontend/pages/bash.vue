<template>
  <section class="container">
    <h1>Bash</h1>

    <p class="intro">Blaine and Ling Ling are having a very intimate wedding in Estes Park, CO on 7/6/2019. 
    However, they had missed so many people who had a large impact on their lives. 
    So a big "Thank You!" goes out to Blaine's generous father, Bill Price, 
    as he has decided to help the two host a 2nd local event with everyone in it. 
    We are so pleased to invite you all to our casual local post wedding get together!</p>

    <Card>
      <h2>Venue</h2>
      <img 
        alt="bash 2 menu"
        src="/bash-2-venue.jpg">

      <p>Please join us around <time>Saturday, August 31, 2019 at 5pm at this address:</time></p>
      <address>
        <a href="https://goo.gl/maps/dS2sknSV7DQtGxQ7A">
          3181 Derby Ln Williamsburg,<br>
          Virginia 23185
        </a>
      </address>
    </Card>

    <Card>
      <h2>Lodging</h2>

      <p>All recommended hotels have over 4/5 user ratings on Google Maps</p>

      <table>
        <thead>
          <th>Fanciness</th>
          <th>Name</th>
          <th>Directions</th>
        </thead>
          
        <tbody>
          <tr>
            <td>★★</td>
            <td><a href="https://www.choicehotels.com/virginia/williamsburg/econo-lodge-hotels/va064">Econo Lodge Colonial</a></td>
            <td><a href="https://goo.gl/maps/nRCJzbNb4mKUzDGH7">🏨</a></td>
          </tr>
          <tr>
            <td>★★★</td>
            <td><a href="https://www.colonialwilliamsburghotels.com/accommodations/williamsburg-woodlands/">Williamsburg Woodlands Hotel & Suites</a></td>
            <td><a href="https://goo.gl/maps/NVWf6V53NsuWhWtd6">🏨</a></td>
          </tr>
          <tr>
            <td>★★★★</td>
            <td><a href="https://www.marriott.com/hotels/travel/phfvi-marriotts-manor-club-at-fords-colony/">Marriott's Manor Club at Ford's Colony</a></td>
            <td><a href="https://goo.gl/maps/NyiDbTgNam2tnJxo6">🏨</a></td>
          </tr>
          <tr>
            <td>★★★★★</td>
            <td><a href="https://www.colonialwilliamsburghotels.com/accommodations/williamsburg-inn/">Williamsburg Inn</a></td>
            <td><a href="https://goo.gl/maps/UNen6K4hNsWBZ5PY9">🏨</a></td>
          </tr>
        </tbody>
      </table>
    </Card>

  </section>
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
.intro {
  font-size: 1.25em;
  margin-bottom: 1.5em;
}

table td,
table th {
  padding: 0 1em;
}
</style>
