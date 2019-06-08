importScripts('/_nuxt/workbox.4c4f5ca6.js')

workbox.precaching.precacheAndRoute([
  {
    "url": "/_nuxt/00a8bdbfb0ab38f9bbb4.js",
    "revision": "ff9ec8b03c20f6d3b85f337c6a69f663"
  },
  {
    "url": "/_nuxt/0298b25bc2636f8ae23a.js",
    "revision": "5e2f5f227735a848465ec9fbd9ce6619"
  },
  {
    "url": "/_nuxt/08655c856b570214255a.js",
    "revision": "878c514cc822c026183beb1572ba693a"
  },
  {
    "url": "/_nuxt/0e4d5e11361842728f5a.js",
    "revision": "4a4b1a652f4a38dc99ec8f393f687e08"
  },
  {
    "url": "/_nuxt/18f08a6cfb558fbdd023.js",
    "revision": "f376026fc2aea4e04c1d05cc3764d1c7"
  },
  {
    "url": "/_nuxt/38ddc6c0567536a77e10.js",
    "revision": "ecd6269fbf8e0738d98c2a76771fd6aa"
  },
  {
    "url": "/_nuxt/46ab6099d189f3569dda.js",
    "revision": "ae7bc9b392a0ff7503fd1aa9b6a8e551"
  },
  {
    "url": "/_nuxt/7a0f22badd11d4ddf02e.js",
    "revision": "71d01073e9e241495d1b3a9c00e8cb1b"
  },
  {
    "url": "/_nuxt/8173e45b881147e5889b.js",
    "revision": "8066c85490ab0e5b1144bd5f7a427b0d"
  },
  {
    "url": "/_nuxt/9087492df32eddcf66f4.js",
    "revision": "9dac835f8d9445b47a5216778c6a669b"
  },
  {
    "url": "/_nuxt/b20876dbaedfa0a6f01b.js",
    "revision": "968fcd3eaeef655f79845bf68c515dcd"
  },
  {
    "url": "/_nuxt/c73dfe3c017b18b67fe3.js",
    "revision": "ec18b8b525a67a6234f64cbe9259ce21"
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
