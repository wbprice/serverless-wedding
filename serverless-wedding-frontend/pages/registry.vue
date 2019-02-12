<template>
  <section class="registry">
    <div class="container">
      <h1>Registry</h1>

      <section class="poem">
        <p>Our life together has begun
        We have almost everything under the sun.
        Your presence would be a gift
        But if you insist on giving us a lift
        We would like to spread our adventurous wings
        With the help of some honeymoon bling
        Our holiday will surely be a dream come true 
        All from the THANKS of YOU!</p>

        <p>For those of who can’t come to town
        Please don’t let that get you down
        We will celebrate round two with you!</p>
      </section>

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
          content: `What better place to get a good nights rest in a rustic cabin next to the moutains!
          We will definitely appreciate your generosity when we crash in after a long day of hiking!`,
          cost: '$129.99',
          buttonText: 'Buy It',
          image: 'lodging'
        },
        {
          title: 'Roundtrip Air Fare',
          cost: '$218.88',
          content: `As much as we love traveling, airfare is an inevitable and large expense of our trip. 
          Fly us to the wedding, honeymoon, and back home.  We will definitely appreciate your generosity when we land safely at our destination!`,
          buttonText: 'Buy It',
          image: 'airfare'
        },
        {
          title: 'Sightseeing Tours',
          cost: '$78.88',
          content: `Spending the afternoon seeing the sights and being true tourists.
          We will definitely appreciate your generosity when we are exploring all the sights!`,
          buttonText: 'Buy It',
          image: 'sightseeing'
        },
        {
          title: 'Romantic Dinner',
          cost: '$99.99',
          content: `Dinner and drinks to refuel after a day of hiking. 
          We will definitley appreciate your generosity when we are enjoying the good foods like a true foodie should!`,
          buttonText: 'Buy It',
          image: 'dining'
        },
        {
          title: 'Hang Gliding',
          cost: '$168.88',
          content: `To fufill the adventurous spirits within. 
          We will definitley appreciate your generosity when we are soaring amongst the birds!`,
          buttonText: 'Buy It',
          image: 'parasailing'
        },
        {
          title: 'Spa for Two',
          cost: '$68.88',
          content: `Massage and relaxation would be a wonderful way of recovery after some long hikes. 
          We will definitley appreciate your generosity when I am one with Jello.`,
          buttonText: 'Buy It',
          image: 'spa'
        },
        {
          title: 'Horseback Riding',
          cost: '$88.88',
          content: `Horseback ride on some beautiful trails.  We will definitley appreciate your generosity when we are 
          soaking in the beautiful weather and views.`,
          buttonText: 'Buy It',
          image: 'horseback-riding'
        },
        {
          title: 'Dancing',
          cost: '$29.99',
          content: `Who doesn't like to boogie woogie woogie? 
          We will definitley appreciate your generosity when the music makes us groovy.`,
          buttonText: 'Buy It',
          image: 'dance'
        },
        {
          title: 'Lucky Envelope',
          cost: 'Tell Us What You Want Us To Do!',
          content: 'The content',
          buttonText: 'Buy It'
        }
      ]
    }
  }
}
</script>

<style>
.poem {
  font-size: 1.25em;
}
</style>
