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
    "url": "/_nuxt/49e2cadf7c8fd7e19430.js",
    "revision": "4e5a37473c38941682a55557ec0d4a59"
  },
  {
    "url": "/_nuxt/4daf74fbb016ad15964e.js",
    "revision": "6977aab08fd8ce144e9041216c346b42"
  },
  {
    "url": "/_nuxt/7ee8833b25c235834853.js",
    "revision": "c1ec8efd58bf97dad0d77b66ac58eaa0"
  },
  {
    "url": "/_nuxt/81151174fcd97a99dd86.js",
    "revision": "25382ddacf54f21ac7cb3aa297d3fe61"
  },
  {
    "url": "/_nuxt/b71d38d85711b5446180.js",
    "revision": "00c87f0d3e26a31724b21a719a1933c7"
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
    "url": "/_nuxt/c74aef6353b229795c53.js",
    "revision": "5440e2ea93c3cb200d5e06f84b4b6c24"
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
