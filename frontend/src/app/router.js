/**
 * vue-router init file.
 */

import Vue from 'vue'
import Router from 'vue-router'

Vue.use(Router)

export function createRouter () {
  return new Router({
    mode: 'history',
    base: '/app',
    routes: [
      {
        name: 'calories',
        path: '/calories',
        // component: () => import('../components/pages/Calories.vue'), // --- это используется для lazy loading
        component: require('../components/pages/Calories.vue').default,
      },
      {
        name: 'index',
        path: '/',
        component: require('../components/pages/Index.vue').default,
      },
    ]
  })
}
