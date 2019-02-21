importScripts('/_nuxt/workbox.4c4f5ca6.js')

workbox.precaching.precacheAndRoute([
  {
    "url": "/_nuxt/04ad5c52db5f1cb9817b.js",
    "revision": "094fc08a9d5d3cffc9032ddc2f5c8784"
  },
  {
    "url": "/_nuxt/164cf549c640aabd5c84.js",
    "revision": "67b6ef7d50cc5bd6e5bf2f95c9ceb6bd"
  },
  {
    "url": "/_nuxt/2003ceb12b1e4d58fef8.js",
    "revision": "b6211dd40b313522950994e419d90cdd"
  },
  {
    "url": "/_nuxt/2c5e989e90ed6f084464.js",
    "revision": "1508aed648dc95da34a56d749bdd7b75"
  },
  {
    "url": "/_nuxt/3052067531db34d82b31.js",
    "revision": "9d34c1f602beaf0a156569470b7028fc"
  },
  {
    "url": "/_nuxt/3d541b6eba439edfea0e.js",
    "revision": "0d225ae3189cc93390cf413ec16890bb"
  },
  {
    "url": "/_nuxt/4e7a788a1639e5b25806.js",
    "revision": "3a46f3840807865c736785fea1d07dd6"
  },
  {
    "url": "/_nuxt/61e1e3726a9558165db1.js",
    "revision": "1e42552628870b151d79e3ca75426fdc"
  },
  {
    "url": "/_nuxt/bbaa6332a187b8876ee7.js",
    "revision": "896702cc119acb873b965ad301fec913"
  },
  {
    "url": "/_nuxt/d45f30137b1806cbb24d.js",
    "revision": "d6ffe97287b8358754a9ac54b346d4d3"
  },
  {
    "url": "/_nuxt/ffc1149718fad5fd03ae.js",
    "revision": "09118a509a2d6ecc647f85da7f04ef7f"
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
