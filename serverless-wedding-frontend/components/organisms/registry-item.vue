<template>
  <section class="registry-item">
    <h2>{{ title }} <span 
      v-if="cost" 
      class="cost">
      - ${{ cost }}
    </span></h2>
    <img 
      v-if="image" 
      :src="image">
    <p>{{ content }}</p>
    <a 
      :href="link" 
      target="_blank"
      class="button primary">{{ buttonText }}</a>
  </section>
</template>

<script>
const paymentRootUrl = 'https://paypal.me/blinginglove'

export default {
  name: 'RegistryItem',
  props: {
    title: {
      type: String,
      default: 'Activity'
    },
    content: {
      type: String,
      default: `Lorem ipsum dolor sit amet, consectetur adipisicing elit. Incidunt aliquam possimus, voluptatum at, 
    autem vitae praesentium magnam, eligendi cumque quas molestiae quaerat dignissimos ea. Officiis esse id 
    maiores debitis soluta.`
    },
    buttonText: {
      type: String,
      default: 'Buy It!'
    },
    cost: {
      type: String,
      default: null
    },
    image: {
      type: String,
      default: null
    }
  },
  computed: {
    link() {
      if (this.cost) {
        return `${paymentRootUrl}/${this.cost}`
      }
      return paymentRootUrl
    }
  }
}
</script>

<style>
.registry-item {
  display: flex;
  flex-direction: column;
}

.registry-item {
}

.registry-item h2 {
  font-size: 1.25em;
  margin-top: 0;
  margin-bottom: 0.5em;
}

.registry-item img {
  max-width: 75%;
  margin: 0 auto 1em auto;
}

.registry-item .cost {
  color: var(--slate);
}
</style>
