<template>
  <div class="container-fluid">
    <h1>кал ории</h1>
    <div class="row mt-4">
      <div class="col">
        <food-list :store="store" />
      </div>
      <div class="col">
        <food-form :store="store" :food="food" />
      </div>
    </div>

    <div><img src="../../img/money.jpg" alt="Me with money" style="width: 50px;" /></div>
  </div>
</template>

<script>
  import {caloriesStore} from '../../stores/calories';
  import FoodList from "./calories/FoodList";
  import FoodForm from "./calories/FoodForm";

  export default {
    name: 'Calories',
    components: {FoodForm, FoodList},
    data() {
      let store = caloriesStore;
      store.init();

      return {
        store: store,
        food: null
      }
    },
    created() {
        document.title = 'Calories';
    },
  }
</script>

<style lang="scss" scoped>
</style>
