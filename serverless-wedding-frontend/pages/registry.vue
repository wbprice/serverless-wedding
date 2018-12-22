<template>
  <section class="registry">
    <div class="container">
      <h1>Registry</h1>

      <Card 
        v-for="(value, key, index) in registry" 
        :key="index">
        <RegistryItem 
          :key="index"
          :title="value.title"
          :content="value.content"
          :button-text="value.buttonText" />
      </Card>
    </div>
  </section>
</template>

<script>
import RegistryItem from './../components/organisms/registry-item'
import Card from './../components/molecules/card'

const paymentRootUrl = 'https://paypal.me/blinginglove/'

export default {
  components: {
    Card,
    RegistryItem
  },
  data() {
    return {
      registry: [
        {
          title: 'Title',
          content: 'The content',
          link: `${paymentRootUrl}/88.88`,
          buttonText: 'button'
        }
      ]
    }
  }
}
</script>

<style/>
