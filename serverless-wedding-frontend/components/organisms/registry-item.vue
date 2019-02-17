<template>
  <section
    :class="thumbnailStyle"
    class="registry-item">
    <div class="description">
      <h2>{{ title }} <span class="cost">- {{ cost }}</span></h2>
      <p>{{ content }}</p>
      <a 
        :href="link" 
        class="button primary">{{ buttonText }}</a>
    </div>
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
.registry-item {
  display: flex;
  flex-direction: column-reverse;
}

.description {
}

.description h2 {
  margin: 0;
  font-size: 1.25em;
}

.description h3 {
  margin: 0;
}

.description .cost {
  color: var(--slate);
}

.airfare {
  background: url('~assets/airfare.jpg');
}

.dance {
  background: url('~assets/dance.jpg');
}

.dining {
  /* background: url('~assets/dining.jpg'); */
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
