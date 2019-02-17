<template>
  <section class="registry-item">
    <h2>{{ title }} <span class="cost">- {{ cost }}</span></h2>
    <img :src="image">
    <p>{{ content }}</p>
    <a 
      :href="link" 
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
      default: '$88.88'
    },
    image: {
      type: String,
      default: 'dance.jpg'
    }
  },
  computed: {
    link() {
      return `${paymentRootUrl}/${this.cost}`
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
  margin: 0;
  font-size: 1.25em;
}

.registry-item img {
  max-width: 100%;
}

.registry-item h3 {
  margin: 0;
}

.registry-item .cost {
  color: var(--slate);
}
</style>
