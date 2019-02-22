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
    "url": "/_nuxt/28483e2c358f688d9b27.js",
    "revision": "135c980ac4a772231a74ce5f2830d1ee"
  },
  {
    "url": "/_nuxt/3c3bc4f27665a50d1327.js",
    "revision": "68658fb744d12dc03f03ad0f45995a12"
  },
  {
    "url": "/_nuxt/3d541b6eba439edfea0e.js",
    "revision": "0d225ae3189cc93390cf413ec16890bb"
  },
  {
    "url": "/_nuxt/424b440a03be3ef6400f.js",
    "revision": "9a711367f72613630093ce67bc610144"
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
    "url": "/_nuxt/f414e1ec30c2009c60e3.js",
    "revision": "7743be283978a1456409c60e7c8a8f2b"
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
