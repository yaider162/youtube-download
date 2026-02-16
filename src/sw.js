let useCat = false;
self.addEventListener('install', event => {
    console.log('Service worker instalando');
    // en cache
    event.waitUntil(
        caches.open('cacheApp').then(cache => cache.add('cat.svg'))
    );
});

self.addEventListener('activate', event => {
    console.log('Service worker ready');
});

self.addEventListener('message', event => {
    if (event.data.action === 'switchToCat') { useCat = true; }
});

self.addEventListener('fetch', event => {
    const url = new URL(event.request.url);
    if (url.pathname.includes('dog.svg') && useCat) {
        event.respondWith(caches.match('cat.svg'));
    } else {
        event.respondWith(fetch(event.request));
    }
});