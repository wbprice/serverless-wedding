importScripts('/_nuxt/workbox.4c4f5ca6.js')

workbox.precaching.precacheAndRoute([
  {
    "url": "/_nuxt/05cdb63c4236607c5544.js",
    "revision": "9bba63ffcc7043b5d700d96a283682ac"
  },
  {
    "url": "/_nuxt/0abc7b16d807a2ee5dc5.js",
    "revision": "892f08224a26dfec41ae81ab1896a046"
  },
  {
    "url": "/_nuxt/0b122c78c9e50741f312.js",
    "revision": "f77bb3e399335bb7ae085e2222460cd4"
  },
  {
    "url": "/_nuxt/12f84eac744a4b1189aa.js",
    "revision": "9bba52f6a2a53a71e17bebc0a5b5cf97"
  },
  {
    "url": "/_nuxt/3c4bb161e10691d35254.js",
    "revision": "3b20f0354c142fe5bd76fd194c022b29"
  },
  {
    "url": "/_nuxt/74d190a7481ac44ecd90.js",
    "revision": "e023dd8293a0daee574f5e2130b1696e"
  },
  {
    "url": "/_nuxt/96bc2b2ea4f9ffb91598.js",
    "revision": "4d0922f2530f1657334ed8cdf633a840"
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
    "url": "/_nuxt/cbd83d8dd844042498d9.js",
    "revision": "8d8f349ea02d2797e1ccb2cc367f5696"
  },
  {
    "url": "/_nuxt/ff5ba55bcc86b1838783.js",
    "revision": "1979e1450babb1787e121463d9965184"
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
