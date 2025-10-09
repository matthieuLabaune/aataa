// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: '2025-07-15',
    devtools: { enabled: true },
    ssr: false,
    app: { 
        baseURL: './',
        head: {
            link: [
                { rel: 'stylesheet', href: '/main.css' }
            ]
        }
    },
    devServer: { host: '127.0.0.1', port: 3000 },
    nitro: { preset: 'static' },
})
