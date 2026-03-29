import { createRouter, createWebHashHistory } from 'vue-router'
import MainWindow from '../views/MainWindow.vue'

import GameView from '../views/GameView.vue'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: MainWindow,
    },
    {
      path: '/game',
      name: 'game',
      component: GameView,
    },
  ],
})

export default router
