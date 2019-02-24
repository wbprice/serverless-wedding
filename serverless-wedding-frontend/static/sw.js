importScripts('/_nuxt/workbox.4c4f5ca6.js')

workbox.precaching.precacheAndRoute([
  {
    "url": "/_nuxt/05cdb63c4236607c5544.js",
    "revision": "9bba63ffcc7043b5d700d96a283682ac"
  },
  {
    "url": "/_nuxt/12f84eac744a4b1189aa.js",
    "revision": "9bba52f6a2a53a71e17bebc0a5b5cf97"
  },
  {
    "url": "/_nuxt/1b0e98eb5d802646634a.js",
    "revision": "61f0c7b5d38ac20ca52543844985fc53"
  },
  {
    "url": "/_nuxt/23e6c0a19ca8b3e8021c.js",
    "revision": "d63084c6773a7467d8ead9a479dbfc1f"
  },
  {
    "url": "/_nuxt/6a58dc06551a5d94e5d7.js",
    "revision": "74fc831cb928a25b9a67356e5c689f39"
  },
  {
    "url": "/_nuxt/6d5a471e85b6301cb6a1.js",
    "revision": "bbc63ca5c665e09431f2147e20ebd2d3"
  },
  {
    "url": "/_nuxt/74d190a7481ac44ecd90.js",
    "revision": "e023dd8293a0daee574f5e2130b1696e"
  },
  {
    "url": "/_nuxt/aa66befc187319e12dea.js",
    "revision": "ed000ccb0418b0c8f0fc9dcd440572a9"
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
