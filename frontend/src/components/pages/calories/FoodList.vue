<template>
  <div>
    <h3>Food list</h3>
    <table class="table align-middle">
      <thead>
        <tr>
          <td>Название</td>
          <td>Белки</td>
          <td>Жиры</td>
          <td>Углеводы</td>
          <td>Калории</td>
          <td>Масса порции</td>
          <td>Цена порции</td>
          <td></td>
        </tr>
      </thead>
      <tbody>
        <tr v-for="food in store.foods">
          <td>{{food.name}}</td>
          <td>{{food.proteins}}</td>
          <td>{{food.fats}}</td>
          <td>{{food.carbos}}</td>
          <td>{{food.kcals}}</td>
          <td>{{food.weight}}</td>
          <td>{{food.price}}</td>
          <td>
            <button type="button" class="btn btn-outline-primary btn-sm" @click="edit(food)">Edit</button>
            <button type="button" class="btn btn-danger btn-sm" @click="del(food)">Del</button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
  export default {
    name: "FoodList",
    props: ['store'],
    data(){
      return {
      }
    },
    methods: {
      edit(food) {
        this.store.setForm(food);
      },
      del(food) {
        this.store.deleteFood(food.id)
      }
    }
  }
</script>

<style lang="scss" scoped>

</style>
