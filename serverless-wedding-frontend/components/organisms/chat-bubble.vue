<template>
  <div 
    :class="{reply: isReply}"
    class="chat">
    <div :class="profilePicClass" />
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
  },
  computed: {
    profilePicClass: function() {
      const obj = {
        'chat-user-photo': true
      }

      switch (this.heading) {
        case 'Ling Ling':
          obj['ling'] = true
          break
        case 'Blaine':
          obj['blaine'] = true
          break
        default:
          obj['default'] = true
      }

      return obj
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
  box-shadow: inset 0 0 0 0.25em var(--white);
}

.blaine.chat-user-photo {
  background: url('./../../static/blaine-profile.jpg');
  background-size: contain;
}

.ling.chat-user-photo {
  background: url('./../../static/cynthia-profile.jpg');
  background-size: contain;
}

.default.chat-user-photo {
  background: url('./../../static/isabelle-profile.png');
  background-size: contain;
}

.chat.reply .chat-user-photo {
  border-radius: 100% 100% 100% 0%;
}

.chat-bubble {
  text-align: left;
  flex: 1;
  background: var(--white);
  color: var(--darkslate);
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
}

.chat-bubble-blaine {
  text-align: left;
}

.chat-bubble-ling {
  text-align: right;
  align-self: flex-end;
}
</style>
