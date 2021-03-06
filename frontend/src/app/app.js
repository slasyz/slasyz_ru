/**
 * Vue application init file.
 */

import Vue from 'vue'
import VueRouter from "vue-router";

import { createRouter } from './router'
import App from './App.vue'
import '../styles/app.scss';

export function createApp () {
  Vue.use(VueRouter);

  const router = createRouter()

  const app = new Vue({
    el: '#app',
    render: h => h(App),
    router,
    components: { App }
  })

  // return both the app and the router
  return { app, router }
}
