<template>
  <div>
    <form @submit.prevent="submit" class="row">
      <input type="hidden" v-model="store.form.id" />
      <div class="col-md-12 mb-3">
        <label for="name">Name</label>
        <input type="text" class="form-control" id="name" v-model="store.form.name" />
      </div>
      <div class="col-md-3 mb-3">
        <label for="proteins">Proteins / 100 g</label>
        <input type="number" class="form-control no-spin" id="proteins" v-model="store.form.proteins" step="any" />
      </div>
      <div class="col-md-3 mb-3">
        <label for="fats">Fats / 100 g</label>
        <input type="number" class="form-control no-spin" id="fats" v-model="store.form.fats" step="any" />
      </div>
      <div class="col-md-3 mb-3">
        <label for="carbos">Carbohydrates / 100 g</label>
        <input type="number" class="form-control no-spin" id="carbos" v-model="store.form.carbos" step="any" />
      </div>
      <div class="col-md-3 mb-3">
        <label for="kcals">Kilocalories / 100 g</label>
        <input type="number" class="form-control no-spin" id="kcals" v-model="store.form.kcals" step="any" />
      </div>
      <div class="col-md-6 mb-3">
        <label for="weight">Portion weight</label>
        <input type="number" class="form-control no-spin" id="weight" v-model="store.form.weight" />
      </div>
      <div class="col-md-6 mb-3">
        <label for="price">Portion price</label>
        <input type="number" class="form-control no-spin" id="price" v-model="store.form.price" />
      </div>
      <div class="col-md-12 mt-2">
        <button type="submit" class="btn btn-primary" v-html="store.form.id ? 'Save' : 'Create'"></button>
      </div>
    </form>
  </div>
</template>

<script>
  export default {
    name: "FoodForm",
    props: ['store', 'food'],
    methods: {
      submit(){
        if (this.store.form.id) {
          const food = this.store.editFood(this.store.form);
          alert('saved: ' + food.id);
        } else {
          const food = this.store.addFood(this.store.form);
          alert('created: ' + food.id);
        }
      }
    },
    updated(){
      console.log('is updated');
    },
  }
</script>

<style lang="scss" scoped>
  @import '../../../styles/bootstrap';
</style>
