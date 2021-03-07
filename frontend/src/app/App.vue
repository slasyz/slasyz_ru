<template>
    <div id="app">
      <topbar />
      <transition name="fade">
        <keep-alive>
          <router-view class="pt-2"></router-view>
        </keep-alive>
      </transition>
    </div>
</template>

<script>
    import Topbar from "../components/Topbar.vue";

    export default {
      name: "App",
      components: {Topbar},
    }
</script>

<style lang="scss" scoped>
    @import '../styles/bootstrap.scss';
</style>

