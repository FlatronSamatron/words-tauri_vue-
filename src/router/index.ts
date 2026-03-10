import { createRouter, createWebHistory } from 'vue-router'
import MainWindow from '../views/MainWindow.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: MainWindow,
    },
  ],
})

export default router
