// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: '2025-07-15',
    devtools: { enabled: false },
    ssr: false,
    css: [
        '~/assets/css/tokens.css',
        '~/assets/css/material.css',
        '~/public/main.css'
    ],
    app: {
        baseURL: './',
        head: {
            link: [
                { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
                { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' },
                { rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap' }
            ]
        }
    },
    devServer: {
        host: '127.0.0.1',
        port: 3000,
    },
    nitro: {
        preset: 'static'
    },
    vite: {
        server: {
            watch: {
                ignored: ['**/src-tauri/**', '**/target/**', '**/node_modules/**']
            }
        }
    }
})
