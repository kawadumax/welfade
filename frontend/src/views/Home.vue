<template>
  <div class="home">
    <img alt="Vue logo" src="@/assets/logo.png">
    <p>{{ping}}</p>
  </div>
</template>

<script lang="ts">
import Vue from "vue"
// @ is an alias to /src

export type Data = {
  ping: string
}

export default Vue.extend({
  name: 'Home',
  created(): void {
    this.$axios.get("/ping")
      .then(response => {
        console.log('status:', response.status) // 200
        console.log('body:', response.data)     // response body.)
        this.ping = response.data
      })
      .catch(err => {
        this.ping = 'err:' + err;
    })
  },
  components: {
  },
  data(): Data {
    return {
      ping: "",
    }
  }

})
</script>
