import axios from 'axios';

export let caloriesStore = {
    state: {
        foods: [],
        eatings: [],
        form: {}
    },

    init(){
        this.updateFoods();
        this.resetForm();
    },
    updateFoods() {
        this.foods = this.listFoods();
    },

    resetForm() {
        this.form = {
            id: 0,
            name: '',
            proteins: '',
            fats: '',
            carbos: '',
            kcals: '',
            weight: '',
            price: '',
        }
    },
    setForm(form) {
        this.form = {
            id: form.id,
            name: form.name,
            proteins: form.proteins,
            fats: form.fats,
            carbos: form.carbos,
            kcals: form.kcals,
            weight: form.weight,
            price: form.price,
        }
    },

    listFoods() {
        axios.get('/api/foods/list').then(response => {
            // TODO: error handling
            this.foods = response.data;
        })
    },
    addFood(food) {
        axios.post('/api/foods/add', {food: food}).then(response => {
            // TODO: error handling
            this.updateFoods();
            this.resetForm();
            // TODO: return response.data.id
        })
    },
    editFood(food) {
        axios.post('/api/foods/edit', {food: food}).then(response => {
            // TODO: error handling
            this.updateFoods();
            this.resetForm();
        })
    },
    deleteFood(id) {
        axios.delete('/api/foods/delete', {data: {id: id}}).then(response => {
            // TODO: error handling
            this.updateFoods();
        })
    },
}
