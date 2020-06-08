<template>
  <div class="home">
    <img alt="Vue logo" src="@/assets/logo.png">
    <p>{{user}}</p>
  </div>
</template>

<script lang="ts">
import Vue from "vue"
// @ is an alias to /src

export type Data = {
  user: string
}

export default Vue.extend({
  name: 'Home',
  created(): void {
    this.$axios.get("/user/2")
      .then(response => {
        console.log('status:', response.status) // 200
        console.log('body:', response.data)     // response body.)
        this.user = response.data
      })
      .catch(err => {
        console.log('err:', err);
    })
  },
  components: {
  },
  data(): Data {
    return {
      user: "hoge",
    }
  }

})
</script>
