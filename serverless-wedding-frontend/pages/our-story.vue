<template>
  <div class="our-story container">
    <h1>Our Story</h1>
    <p>(In Chats)</p>

    <dl>
      <div
        v-for="message in messages"
        :key="message.question.content">
        <dt>
          <ChatBubble
            :heading="message.question.heading"
            :content="message.question.content" />
        </dt>
        <dd>
          <ChatBubble
            v-for="answer in message.answers"
            :key="answer.heading"
            :heading="answer.heading"
            :content="answer.content"
            is-reply />
        </dd>
      </div>
    </dl>

  </div>
</template>

<script>
import ChatBubble from './../components/organisms/chat-bubble'

export default {
  components: {
    ChatBubble
  },
  data() {
    return {
      messages: [
        {
          question: {
            heading: 'Wedding Planner',
            content: 'How did you meet?'
          },
          answers: [
            {
              heading: 'Ling Ling',
              content: `Through mutual friends, I had thrown a karaoke party for my birthday and one of my girlfriends invited her coworker friend,
                        he initially thought I was married by the virtue of having a large house, and I thought he was handsome
                        but was too busy hosting.  Later he had an extra ticket to a Punk concert and I said I love music, he
                        3rd wheeled his friend who he originally gave his extra ticket to and bought me another.
                        He picked me up and it was a done deal, mosh pit and all.  I thought he was easy going, mild, gentle,
                        active, and extremely intelligent.`
            },
            {
              heading: 'Blaine',
              content: `She came to a talk I gave at a conference with some friends and I thought she was cute.
                Later on, I had an extra ticket to see Black Flag. I made a post online about how I needed someone to go with me.
                She popped up in the thread, so on the day of the concert (Jul 2nd), I picked her up at her house.
                We were early for the concert, so to kill time we ran up and down the stairs in the parking structure.
                Neither of us were prepared for the noise or the mosh pit, but she stayed cool and looked like she was having fun in the chaos.`
            }
          ]
        },
        {
          question: {
            heading: 'Wedding Planner',
            content: `What made you fall in love with the other person
                      for the first time? Can you describe that moment?`
          },
          answers: [
            {
              heading: 'Ling Ling',
              content: `How thoughtful he is and listens to the fine details, the first time I even mentioned I
                        was a bit chilly walking and talking to another friend he immediately takes off his jacket for me.`
            },
            {
              heading: 'Blaine',
              content: `She was training for a half-marathon, so our second date was running 14 miles through the forest on
                        a Saturday morning. It was a beautiful day and we enjoyed each other's company. When we finished the
                        run, we ate fresh fruit she had prepared that morning. So thoughtful!`
            }
          ]
        },
        {
          question: {
            heading: 'Wedding Planner',
            content: `If you had to pick just one thing that you love the most about the other person... what would it be?`
          },
          answers: [
            {
              heading: 'Ling Ling',
              content: `How he continuously tries to make me smile even when I'm not in a good mood and frowning,
                        he won't stop until he gets my pearly whites showing each time.`
            },
            {
              heading: 'Blaine',
              content: `Cynthia is generous and empathetic. She's always concerned about the well-being of others and gives even when it hurts.`
            }
          ]
        },
        {
          question: {
            heading: 'Wedding Planner',
            content: `How did the proposal happen? Tell me the story!`
          },
          answers: [
            {
              heading: 'Blaine',
              content: `We spent a weekend in Asheville, NC to see the sights and do some hiking to celebrate our
                        2nd year together. On July 2nd, we went hiking in a nearby national park known for its
                        waterfalls. The first few waterfalls we saw were pretty, but not *the one*, so we kept hiking.
                        Eventually, after crawling over rocks and wading through streams, we found the perfect waterfall.
                        Cynthia says I started looking nervous.`
            },
            {
              heading: 'Ling Ling',
              content: `He pulled out a pretty handmade wooden puzzle box we bought at the farmers market the day before.
                        Why did he buy such a complicated box to put a ring in?  Lol.  He pulled out the ring and got on his
                        knees and asked, "will you marry me?"  I saw a few people eyeing us and I said "Only if you show me
                        your best dance moves!"  I'm sure this confused anyone nearby that it might even be a proposal.
                        He threw out a few dance moves and then hurriedly put the ring on my finger even without a response.
                        I was cracking up and laughing non-stop the whole time while he was dancing and putting the ring on my
                        finger. Lol.`
            }
          ]
        },
        {
          question: {
            heading: 'Wedding Planner',
            content: `If you had a free day where you could go and do whatever you wanted, with no limits,
                      what would you do together? Describe your perfect day.`
          },
          answers: [
            {
              heading: 'Ling Ling',
              content: `I would see all of the earths natural wonders and play with all the sweetest animals in their natural
                        habitats at animal sanctuaries holding hands and eating fresh foods, exotic fruits.`
            },
            {
              heading: 'Blaine',
              content: `I want to go tandem hang-gliding with Cynthia in the mountains and then go eat lots of fatty/cheesy
                        foods in a bustling market.`
            }
          ]
        },
        {
          question: {
            heading: 'Wedding Planner',
            content: `What are some unique things your family and friends notice/say about your relationship?`
          },
          answers: [
            {
              heading: 'Ling Ling',
              content: `We are stuck to each other like glue.  We have never seen each other so happy before.
                        They say we have changed for the better.  Our spirits have been lifted.  I have never known this amount
                        of happiness could exist!`
            },
            {
              heading: 'Blaine',
              content: `D'awwwwwwwwwwwwwwwwwwwwww`
            }
          ]
        }
      ]
    }
  }
}
</script>

<style>
dd {
  margin-bottom: 4em;
}

dt {
  margin-bottom: 1em;
}
</style>
