<template>
  <section class="registry">
    <div class="container">
      <h1>Registry</h1>

      <section class="poem">
        <p>Our life together has begun.<br>
          We have almost everything under the sun.<br>
          Your presence would be a gift.<br>
          But if you insist on giving us a lift,<br>
          We would like to spread our adventurous wings<br>
          With the help of some honeymoon bling<br>
          Our holiday will surely be a dream come true<br>
          All thanks to you!</p><br>

        <p>For those of who can’t come to town<br>
          Please don’t let that get you down<br>
          We will celebrate round two with you!</p>
      </section>

      <div class="registry-heading">
        <h2>We'll be travelling all over Colorado for our honeymoon!</h2>
        <p>Here's a short list of some of the things we want to do!</p>
      </div>

      <Card
        v-for="(value, key, index) in registry"
        :key="index">
        <RegistryItem
          :key="index"
          :cost="value.cost"
          :title="value.title"
          :content="value.content"
          :button-text="value.buttonText"
          :image="value.image" />
      </Card>
    </div>
  </section>
</template>

<script>
import RegistryItem from './../components/organisms/registry-item'
import Card from './../components/molecules/card'

export default {
  components: {
    Card,
    RegistryItem
  },
  data() {
    return {
      registry: [
        {
          title: 'Accomodations',
          content: `Is there a better place to get a good night's rest than in a cabin in the mountains? We will appreciate your generosity when we crash after a long day of hiking!`,
          cost: '129.99',
          buttonText: 'Buy It',
          image: '/lodging.jpg'
        },
        {
          title: 'Roundtrip Air Fare',
          cost: '218.88',
          content: `We love traveling, but airfare is expensive. Fly us to the wedding, to the honeymoon, and back home.  We will appreciate your generosity when we land safely at our destination!`,
          buttonText: 'Buy It',
          image: '/airfare.jpg'
        },
        {
          title: 'Sightseeing Tours',
          cost: '78.88',
          content: `We'll spend the afternoon seeing the sights and being tourists. We will appreciate your generosity when we are exploring all the sights!`,
          buttonText: 'Buy It',
          image: '/sightseeing.jpg'
        },
        {
          title: 'Romantic Dinner',
          cost: '99.99',
          content: `Help us buy dinner and drinks to refuel after a day of hiking. We'll appreciate your generosity when we are enjoying good food as a true foodie should!`,
          buttonText: 'Buy It',
          image: '/dining.jpg'
        },
        {
          title: 'Adventure Package',
          cost: '168.88',
          content: `This package will help us to satisfy our adventurous spirits. We will appreciate your generosity when we are soaring amongst the birds!`,
          buttonText: 'Buy It',
          image: '/parasailing.jpg'
        },
        {
          title: 'Spa for Two',
          cost: '68.88',
          content: `Getting a massage will be a wonderful way to recover after a long hike. We will appreciate your generosity when we are one with Jello.`,
          buttonText: 'Buy It',
          image: '/spa.jpg'
        },
        {
          title: 'Horseback Riding',
          cost: '88.88',
          content: `Help us buy a horseback ride on beautiful trails. We will appreciate your generosity when we are soaking in the beautiful weather and views.`,
          buttonText: 'Buy It',
          image: '/horseback-riding.jpg'
        },
        {
          title: 'Dancing',
          cost: '29.99',
          content: `Who doesn't like to boogie-woogie? We will appreciate your generosity when the music makes us groovy.`,
          buttonText: 'Buy It',
          image: '/dance.jpg'
        },
        {
          title: 'Lucky Envelope',
          content: 'Tell Us What You Want Us To Do!',
          buttonText: 'Buy It',
          image: '/lucky-envelope.jpg'
        }
      ]
    }
  }
}
</script>

<style>
.poem {
  font-size: 1.25em;
  margin-bottom: 1.5em;
}

.registry-heading {
  margin-bottom: 1em;
}

.registry-heading p {
  font-size: 1.25em;
}
</style>
