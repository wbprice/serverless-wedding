importScripts('/_nuxt/workbox.4c4f5ca6.js')

workbox.precaching.precacheAndRoute([
  {
    "url": "/_nuxt/16699e596e90c3c25166.js",
    "revision": "dd6739747f86c56eca6ba29050006a08"
  },
  {
    "url": "/_nuxt/35c206e1fed76d7d606b.js",
    "revision": "6292550182bd3da674ea3630e6035e7c"
  },
  {
    "url": "/_nuxt/46ab6099d189f3569dda.js",
    "revision": "ae7bc9b392a0ff7503fd1aa9b6a8e551"
  },
  {
    "url": "/_nuxt/49bf0af30579827fda39.js",
    "revision": "858425198d1374c375f42993e405358a"
  },
  {
    "url": "/_nuxt/4ccb8e71f6713ab7800d.js",
    "revision": "3fdf8eb79b347bfd8ff7b251c03215ac"
  },
  {
    "url": "/_nuxt/68f2b319afbe951c0917.js",
    "revision": "7169e78dd8dab30a0da6eb907a71b058"
  },
  {
    "url": "/_nuxt/81db73349d920243078b.js",
    "revision": "cd7f9939123adb5425fefb651f443468"
  },
  {
    "url": "/_nuxt/9620967105d2f7cda449.js",
    "revision": "e93d1b0bdc72863d42e0704c0043df53"
  },
  {
    "url": "/_nuxt/d0a9fe81018f5ccdcac9.js",
    "revision": "5af4fc400f23eac25ca715257e258f99"
  },
  {
    "url": "/_nuxt/d32d65f3fd61264675f4.js",
    "revision": "f857f1ca5425d2efa552f5db2ab4fc4b"
  },
  {
    "url": "/_nuxt/f17e6cdc3b92a0cc8189.js",
    "revision": "3663f2e49e51f31cc601c3f60f978a3b"
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
