importScripts('/_nuxt/workbox.4c4f5ca6.js')

workbox.precaching.precacheAndRoute([
  {
    "url": "/_nuxt/03192d4cdb6de008ad1b.js",
    "revision": "ac82aacb7850aa7d74b9782f37798e3b"
  },
  {
    "url": "/_nuxt/12a2cac3afcb26af5b9c.js",
    "revision": "50dd1906f86c4ad6ac6cf39b2da0108e"
  },
  {
    "url": "/_nuxt/12f84eac744a4b1189aa.js",
    "revision": "9bba52f6a2a53a71e17bebc0a5b5cf97"
  },
  {
    "url": "/_nuxt/4f55057c2a0582a04314.js",
    "revision": "4bf88f337ea15933f258a9f1f1815cd1"
  },
  {
    "url": "/_nuxt/704b44419f9a0304d171.js",
    "revision": "68db3ad9375675c7c5ee3267ee6f86d3"
  },
  {
    "url": "/_nuxt/8beab813c9f847e2e1c4.js",
    "revision": "f33d49cce2705ca881c93626ec0cebaf"
  },
  {
    "url": "/_nuxt/a43b489a0bc71e088325.js",
    "revision": "30cd0c31324b19c9254573650cf3085a"
  },
  {
    "url": "/_nuxt/bbaa6332a187b8876ee7.js",
    "revision": "896702cc119acb873b965ad301fec913"
  },
  {
    "url": "/_nuxt/c08073998d1cd7f82ec7.js",
    "revision": "b407a3cecc69f93682126b98c4f60313"
  },
  {
    "url": "/_nuxt/cd0c59bb2e177e160de3.js",
    "revision": "d409fa0b96384c836fe626b5eb1aea5b"
  },
  {
    "url": "/_nuxt/d9a03f3f97a6649a9140.js",
    "revision": "152ccef16ceaf153e78e9a310f7727a7"
  }
], {
  "cacheId": "blinging.love",
  "directoryIndex": "/",
  "cleanUrls": false
})

workbox.clientsClaim()
workbox.skipWaiting()

workbox.routing.registerRoute(new RegExp('/_nuxt/.*'), workbox.strategies.cacheFirst({}), 'GET')

workbox.routing.registerRoute(new RegExp('/.*'), workbox.strategies.networkFirst({}), 'GET')

workbox.routing.registerRoute(new RegExp('https://cdnjs.cloudflare.com/ajax/libs/skeleton/.*'), workbox.strategies.cacheFirst({"cacheableResponse":{"statuses":[0,200]}}), 'GET')

workbox.routing.registerRoute(new RegExp('https://fonts.googleapis.com/.*'), workbox.strategies.cacheFirst({"cacheableResponse":{"statuses":[0,200]}}), 'GET')
