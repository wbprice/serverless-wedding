<template>
  <div class="our-story container">
    <h1>Our Story (in chats)</h1>

    <dl class="chat">
      <dt>
        <ChatBubble
          heading="Wedding Planner"
          content="How did you two meet?" />
      </dt>
      <dd>
        <ChatBubble
          reply
          heading="Blaine"
          content="Through mutual friends, I had thrown a karaoke party for my birthday and one of my girlfriends invited her coworker friend, he initially thought I was married by the virtue of having a large house, and I thought he was handsome but was too busy hosting.  Later he had an extra ticket to a Punk concert and I said I love music, he 3rd wheeled his friend who he originally gave his extra ticket to and bought me another.  He picked me up and it was a done deal, mosh pit and all.  I thought he was easy going, mild, gentle, active, and extremely intelligent" />
        <ChatBubble 
          reply
          heading="Ling"
          content="She came to a talk I gave at a conference with some friends and I thought she was cute. Later on, I had an extra ticket to see Black Flag. I made a post online about how I needed someone to go with me. She popped up in the thread, so on the day of the concert (Jul 2nd), I picked her up at her house. We were early for the concert, so to kill time we ran up and down the stairs in the parking structure. Neither of us were prepared for the noise or the mosh pit, but she stayed cool and looked like she was having fun in the chaos." />
      </dd>
    </dl>

  </div>
</template>

<script>
import ChatBubble from './../components/organisms/chat-bubble'

export default {
  components: {
    ChatBubble
  }
}
</script>

<style>
.chat dd {
  display: flex;
  flex-direction: column;
}
</style>
