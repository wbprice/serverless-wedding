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
          :link="value.link"
          :button-text="value.buttonText" />
      </Card>
    </div>
  </section>
</template>

<script>
import RegistryItem from './../components/organisms/registry-item'
import Card from './../components/molecules/card'

const paymentRootUrl = 'https://paypal.me/blinginglove'

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
          content: 'The content',
          cost: '$129.99',
          link: `${paymentRootUrl}/129.99`,
          buttonText: 'Buy It'
        },
        {
          title: 'Roundtrip Air Fare',
          cost: '$218.88',
          content: 'The content',
          link: `${paymentRootUrl}/218.88`,
          buttonText: 'Buy It'
        },
        {
          title: 'Sightseeing Tours',
          cost: '$78.88',
          content: 'The content',
          link: `${paymentRootUrl}/78.88`,
          buttonText: 'Buy It'
        },
        {
          title: 'Romantic Dinner',
          cost: '$99.99',
          content: 'The content',
          link: `${paymentRootUrl}/99.99`,
          buttonText: 'Buy It'
        },
        {
          title: 'Hang Gliding',
          cost: '$168.88',
          content: 'The content',
          link: `${paymentRootUrl}/168.88`,
          buttonText: 'Buy It'
        },
        {
          title: 'Spa for Two',
          cost: '$68.88',
          content: 'The content',
          link: `${paymentRootUrl}/68.88`,
          buttonText: 'Buy It'
        },
        {
          title: 'Horseback Riding',
          cost: '$88.88',
          content: 'The content',
          link: `${paymentRootUrl}/88.88`,
          buttonText: 'Buy It'
        },
        {
          title: 'Dancing',
          cost: '$29.99',
          content: 'The content',
          link: `${paymentRootUrl}/29.99`,
          buttonText: 'Buy It'
        },
        {
          title: 'Lucky Envelope',
          cost: 'Tell Us What You Want To Do!',
          content: 'The content',
          link: `${paymentRootUrl}`,
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
