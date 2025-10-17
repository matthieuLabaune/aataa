import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Trash from '../views/Trash.vue'
import Settings from '../views/Settings.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/trash',
      name: 'trash',
      component: Trash
    },
    {
      path: '/settings',
      name: 'settings',
      component: Settings
    }
  ]
})

export default router
