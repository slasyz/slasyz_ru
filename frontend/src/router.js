/**
 * vue-router init file.
 */

import Router from 'vue-router'


export function createRouter () {
  return new Router({
    mode: 'history',
    base: '/app',
    routes: [
      {
        name: 'calories',
        path: '/calories/', // TODO: router-link генерирует ссылку без слэша
        // component: () => import('../components/pages/Calories.vue'), // --- это используется для lazy loading
        component: require('./components/pages/Calories.vue').default,
      },
      {
        name: 'index',
        path: '/',
        component: require('./components/pages/Index.vue').default,
      },
    ]
  })
}
