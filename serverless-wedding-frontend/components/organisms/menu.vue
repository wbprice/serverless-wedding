<template>
  <nav class="container menu">
    <ul>
      <li 
        v-for="item in items" 
        :key="item.route">
        <nuxt-link :to="item.route">{{ item.label }}</nuxt-link>
      </li>
    </ul>
  </nav>
</template>

<script>
export default {
  name: 'Menu',
  data() {
    return {
      items: [
        { route: '/', label: 'Home' },
        { route: '/rsvp', label: 'RSVP' },
        { route: '/registry', label: 'Registry' },
        { route: '/our-story', label: 'Our Story' },
        { route: '/logistics', label: 'Logistics' }
      ]
    }
  }
}
</script>

<style>
.menu {
  border-top: 1px solid var(--offwhite);
  border-bottom: 1px solid var(--offwhite);
  padding: 0.5em 0em;
  margin-bottom: 2em;
}
.menu ul {
  display: flex;
  justify-content: center;
  margin: 0;
}
.menu li {
  list-style: none;
  margin: 0em 0.5em;
  font-size: 1.25em;
}
.menu li a {
  color: var(--white);
}
</style>
