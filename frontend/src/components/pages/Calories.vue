<template>
    <div>
        <h1>кал ории</h1>
        <img src="../../img/money.jpg" alt="Me with money" />
    </div>
</template>

<script>
    export default {
        name: "Calories",
        created() {
            document.title = 'Calories';
        }
    }
</script>

<style lang="scss" scoped>
    $color: blue;

    h1 {
        color: $color;
    }
</style>
