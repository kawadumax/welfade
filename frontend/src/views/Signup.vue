<template>
  <v-row align="center" justify="center">
    <v-form ref="form" v-model="valid">
      <v-text-field v-model="name" :counter="10" :rules="nameRules" label="Name" required></v-text-field>
      <v-text-field v-model="email" :rules="emailRules" label="E-mail" required></v-text-field>
      <v-text-field v-model="password" :counter="true" :type="'password'" :rules="passwordRules" label="Password" required></v-text-field>
      <v-text-field v-model="passwordConfirm" :type="'password'" :counter="true" :rules="passwordConfirmRules" label="Password Confirmation" required></v-text-field>
      <v-btn :disabled="!valid" color="success" class="mr-4" @click="submit">Submit</v-btn>
    </v-form>
  </v-row>
</template>

<script lang="ts">
import Vue from "vue";

export type Data = {
  valid: boolean,
  name: string,
  nameRules: any,
  email: string,
  emailRules: any,
  password: string,
  passwordRules: any,
  passwordConfirm: string,
  passwordConfirmRule: any,
}

// @ is an alias to /src
export default Vue.extend({
  name: "Signup",
  components: {
    // HelloWorld
  },
  data: () => ({
    valid: true,
    name: "",
    email: "",
    password: "",
    passwordConfirm: "",
  }),
  computed:{
    form() :any {
      return this.$refs.form;
    },
    passwordConfirmRules() :any { return [
      (v: string) => !!v || "Password Confirmation is required",
      (v: string) => (v && v == this.password) || "Password Confirmation must be same with password"
    ]},
    passwordRules() :any { return [
      (v: string) => !!v || "Password is required",
      (v: string) => (v && v.length > 8) || "Password must be more than 8 characters"
    ]},
    emailRules() :any { return [
      (v: string) => !!v || "E-mail is required",
      (v: string) => /.+@.+\..+/.test(v) || "E-mail must be valid"
    ]},
    nameRules() :any { return [
      (v: string) => !!v || "Name is required",
      (v: string) => (v && v.length <= 10) || "Name must be less than 10 characters"
    ]},
  },
  methods: {
    submit(){
      console.log("Submit")
      this.$axios.post("http://127.0.0.1:3000/user", {
        name: this.name,
        email: this.email,
        password: this.password
      }).then(response => {
        console.log('status:', response.status) // 200
        console.log('body:', response.data)     // response body.)
        this.user = response.data
      }).catch(err => {
        console.log('err:', err);
      })
    },
    validate() {
      this.form.validate();
    },
    reset() {
      this.form.reset();
    },
    resetValidation() {
      this.form.resetValidation();
    }
  }
});
</script>