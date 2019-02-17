<template>
  <div 
    :class="{reply: isReply}"
    class="chat">
    <div class="chat-user-photo" />
    <div class="chat-bubble">
      <div class="chat-bubble-metadata">
        {{ heading }}
      </div>
      <div class="chat-bubble-content">
        {{ content }}
      </div>
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
    },
    isReply: {
      type: Boolean,
      default: false
    }
  }
}
</script>

<style>
.chat {
  display: flex;
  align-items: flex-end;
  margin: 0.5em 0;
}

.chat.reply {
  flex-direction: row-reverse;
}

.chat-user-photo {
  width: 5em;
  height: 5em;
  border-radius: 100% 100% 0 100%;
  background: url('./../../static/isabelle-profile.png'), var(--red);
  background-size: cover;
  box-shadow: inset 0 0 0 0.25em var(--white);
}

.chat.reply .chat-user-photo {
  border-radius: 100% 100% 100% 0%;
}

.chat-bubble {
  text-align: left;
  flex: 1;
  background: var(--white);
  max-width: 75%;
  border-radius: 1em 1em 1em 0;
  padding: 0.75em;
  margin: 0 0.5em;
}

.chat.reply .chat-bubble {
  border-radius: 1em 1em 0em 1em;
}

.chat-bubble .chat-bubble-metadata {
  font-weight: bold;
  text-transform: uppercase;
  font-size: 0.75em;
}

.chat-bubble-blaine {
  text-align: left;
}

.chat-bubble-ling {
  text-align: right;
  align-self: flex-end;
}
</style>
