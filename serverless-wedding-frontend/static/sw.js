importScripts('/_nuxt/workbox.4c4f5ca6.js')

workbox.precaching.precacheAndRoute([
  {
    "url": "/_nuxt/03192d4cdb6de008ad1b.js",
    "revision": "ac82aacb7850aa7d74b9782f37798e3b"
  },
  {
    "url": "/_nuxt/12f84eac744a4b1189aa.js",
    "revision": "9bba52f6a2a53a71e17bebc0a5b5cf97"
  },
  {
    "url": "/_nuxt/378dd75b5a237c7a2fd4.js",
    "revision": "a4db1ab7d6a13a777064ea71c7722059"
  },
  {
    "url": "/_nuxt/704b44419f9a0304d171.js",
    "revision": "68db3ad9375675c7c5ee3267ee6f86d3"
  },
  {
    "url": "/_nuxt/762cd7aafb3c97cec74a.js",
    "revision": "121dc2b6b3c467bc2702d8e5698fd0e7"
  },
  {
    "url": "/_nuxt/8beab813c9f847e2e1c4.js",
    "revision": "f33d49cce2705ca881c93626ec0cebaf"
  },
  {
    "url": "/_nuxt/8e572fe29235dddfb6ce.js",
    "revision": "eafb41b286c1740133fb87685828f5af"
  },
  {
    "url": "/_nuxt/bbaa6332a187b8876ee7.js",
    "revision": "896702cc119acb873b965ad301fec913"
  },
  {
    "url": "/_nuxt/bd5dac162f56c4065d6f.js",
    "revision": "a2e06f9bcb9e5f111f729c31ec4b928f"
  },
  {
    "url": "/_nuxt/c08073998d1cd7f82ec7.js",
    "revision": "b407a3cecc69f93682126b98c4f60313"
  },
  {
    "url": "/_nuxt/d42cc1c6688083d97ea8.js",
    "revision": "0302cab8ace6ff123d7dce80931de400"
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
