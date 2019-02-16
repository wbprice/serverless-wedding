importScripts('/_nuxt/workbox.4c4f5ca6.js')

workbox.precaching.precacheAndRoute([
  {
    "url": "/_nuxt/2c84db8b876a7811a34c.js",
    "revision": "35fdb2a84995a906909296d1f026b3a3"
  },
  {
    "url": "/_nuxt/344aa32cfd4c984e1466.js",
    "revision": "69b545ad9ae276a24c3a70aa1297a686"
  },
  {
    "url": "/_nuxt/5ecbe1a817c80251c39e.js",
    "revision": "45e85c4743ca12fafff20f639c53130f"
  },
  {
    "url": "/_nuxt/7827784526028750ed37.js",
    "revision": "187b37b59e3b50bca76612d25afd8919"
  },
  {
    "url": "/_nuxt/a48eb91fef159602c296.js",
    "revision": "90ab1d73ffcfcff09de05ea6c92ef5a0"
  },
  {
    "url": "/_nuxt/d3e2f8960f11a5c10cf6.js",
    "revision": "1b1dbb9ba2582e1c9a62f1735d86ef03"
  },
  {
    "url": "/_nuxt/d74d64263724cfa048ee.js",
    "revision": "3637bcd3b9b9da9e8b32f7072a32aac5"
  },
  {
    "url": "/_nuxt/ebcf7d81ab0c8f8f3d81.js",
    "revision": "9873869797abf702a4b0da6339cae0a5"
  },
  {
    "url": "/_nuxt/fe5083a052d17cc1af52.js",
    "revision": "5f80a5a9f0ec1ba2ab54357cf4ae6c2d"
  }
], {
  "cacheId": "serverless-wedding-frontend",
  "directoryIndex": "/",
  "cleanUrls": false
})

workbox.clientsClaim()
workbox.skipWaiting()

workbox.routing.registerRoute(new RegExp('/_nuxt/.*'), workbox.strategies.cacheFirst({}), 'GET')

workbox.routing.registerRoute(new RegExp('/.*'), workbox.strategies.networkFirst({}), 'GET')

workbox.routing.registerRoute(new RegExp('https://fonts.googleapis.com/.*'), workbox.strategies.cacheFirst({"cacheableResponse":{"statuses":[0,200]}}), 'GET')
