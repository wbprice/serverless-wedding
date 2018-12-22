<template>
  <section>
    <h2>{{ title }}</h2>
    <p>{{ content }}</p>
    <a 
      :href="link" 
      class="button">{{ buttonText }}</a>
  </section>
</template>

<script>
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
    link: {
      type: String,
      default: 'http://paypal.me/blinginglove/88.88'
    },
    buttonText: {
      type: String,
      default: 'Buy It!'
    }
  }
}
</script>

<style/>
