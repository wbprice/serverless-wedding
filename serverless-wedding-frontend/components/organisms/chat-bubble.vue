<template>
  <div class="chat-bubble">
    <div class="chat-bubble-metadata">
      {{ heading }}
    </div>
    <div class="chat-bubble-content">
      {{ content }}
    </div>
  </div>
</template>

<script>
export default {
  name: 'ChatBubble',
  props: {
    heading: {
      type: String,
      default: 'Person Name'
    },
    content: {
      type: String,
      default: 'Sample Text'
    }
  }
}
</script>

<style>
.chat-bubble {
  background: var(--white);
  margin: 0.5em 0;
  max-width: 75%;
  border-radius: 1em;
  padding: 0.75em;
}

.chat-bubble-blaine {
  text-align: left;
}

.chat-bubble-ling {
  text-align: right;
  align-self: flex-end;
}
</style>
