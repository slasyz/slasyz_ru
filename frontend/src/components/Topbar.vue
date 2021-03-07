<template>
  <nav class="navbar navbar-expand-lg navbar-light bg-light">
    <div class="container-fluid">
      <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" id="navbarSupportedContent">
        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
          <li class="nav-item">
            <router-link :to="{ name: 'index' }" exact-active-class="active" class="nav-link">Index</router-link>
          </li>
          <li class="nav-item">
            <router-link :to="{ name: 'calories' }" active-class="active" class="nav-link">Calories</router-link>
          </li>
        </ul>
      </div>
    </div>
  </nav>
</template>

<script>
  // import 'bootstrap/js/dist/dropdown';
  // import 'bootstrap';

  export default {
    name: "Topbar"
  }
</script>

<style lang="scss" scoped>
  @import '../styles/bootstrap.scss';
</style>
