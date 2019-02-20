importScripts('/_nuxt/workbox.4c4f5ca6.js')

workbox.precaching.precacheAndRoute([
  {
    "url": "/_nuxt/12f84eac744a4b1189aa.js",
    "revision": "9bba52f6a2a53a71e17bebc0a5b5cf97"
  },
  {
    "url": "/_nuxt/2be36a8ce700d0a8e3f0.js",
    "revision": "b5bde4c2a84823a1d5012979f618d34b"
  },
  {
    "url": "/_nuxt/2faed5b1d352c0d53892.js",
    "revision": "69b5d75ec54b4644d8f4f275e7b3f743"
  },
  {
    "url": "/_nuxt/34bf80c16bf79c9bd615.js",
    "revision": "2b069373f6ba4f4f137b4b7503b4effc"
  },
  {
    "url": "/_nuxt/427c473bbdce82d55ffe.js",
    "revision": "82e1f0c9afac067a2bedc24d87f93a4b"
  },
  {
    "url": "/_nuxt/7cc7d832117de4ee628f.js",
    "revision": "a7df2e4c7d9f9b4d054d4690099240d3"
  },
  {
    "url": "/_nuxt/808a8c31dc74e94bfe9d.js",
    "revision": "9cf80b52b6c1c24cb608d724591a056d"
  },
  {
    "url": "/_nuxt/97f7951dde3db3b8ccd9.js",
    "revision": "5e3f710251d1fa3e6c3cd155d7dca993"
  },
  {
    "url": "/_nuxt/f0a1e9f1fa4071c12d2d.js",
    "revision": "95d227fa5571aeb857fa3fe9bc33d7dd"
  },
  {
    "url": "/_nuxt/f187410b8e2a20350f7f.js",
    "revision": "35e879d00dd47e8064d1e83e0bf6d1e1"
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
