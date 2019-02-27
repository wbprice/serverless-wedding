importScripts('/_nuxt/workbox.4c4f5ca6.js')

workbox.precaching.precacheAndRoute([
  {
    "url": "/_nuxt/05cdb63c4236607c5544.js",
    "revision": "9bba63ffcc7043b5d700d96a283682ac"
  },
  {
    "url": "/_nuxt/0e3fe5327bd9fc98c757.js",
    "revision": "cb237adf0a8748f87d7d664a5b387d5e"
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
    "url": "/_nuxt/687bc1725b186f9e46fb.js",
    "revision": "c9549365a8e299b4213377ac46b4ae1d"
  },
  {
    "url": "/_nuxt/74d190a7481ac44ecd90.js",
    "revision": "e023dd8293a0daee574f5e2130b1696e"
  },
  {
    "url": "/_nuxt/8384488c203e07621cc5.js",
    "revision": "b9d88c9c94f3f4d66eb32c0059d5bef1"
  },
  {
    "url": "/_nuxt/a505ad88529689f4dba0.js",
    "revision": "fe2c3366ee0e3b9da9cfbdf861f7fd1e"
  },
  {
    "url": "/_nuxt/bbaa6332a187b8876ee7.js",
    "revision": "896702cc119acb873b965ad301fec913"
  },
  {
    "url": "/_nuxt/c08073998d1cd7f82ec7.js",
    "revision": "b407a3cecc69f93682126b98c4f60313"
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
