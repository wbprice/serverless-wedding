<template>
  <section>
    <div class="description">
      <h2>{{ title }}</h2>
      <h3>{{ cost }}</h3>
      <p>{{ content }}</p>
      <a 
        :href="link" 
        class="button is-primary">{{ buttonText }}</a>
    </div>
    <div :class="thumbnailStyle" />
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
      default: 'airfare.jpg'
    }
  },
  computed: {
    link() {
      return `${paymentRootUrl}/${this.cost}`
    },
    thumbnailStyle() {
      return {
        [this.image]: true,
        thumbnail: true
      }
    }
  }
}
</script>

<style>
.description {
  text-align: left;
}

.thumbnail {
  width: 600px;
  height: 400px;
}

.airfare {
  background: url('~assets/airfare.jpg');
}

.dance {
  background: url('~assets/dance.jpg');
}

.dining {
  background: url('~assets/dining.jpg');
}

.horseback-riding {
  background: url('~assets/horseback-riding.jpg');
}

.lodging {
  background: url('~assets/lodging.jpg');
}

.parasailing {
  background: url('~assets/parasailing.jpg');
}

.sightseeing {
  background: url('~assets/sightseeing.jpg');
}

.spa {
  background: url('~assets/spa.jpg');
}
</style>
