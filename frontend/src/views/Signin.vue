<template>
  <v-row align="center" justify="center">
    <v-form ref="form" v-model="valid">
      <v-text-field v-model="name" :counter="10" :rules="nameRules" label="Name" required></v-text-field>
      <v-text-field v-model="email" :rules="emailRules" label="E-mail" required></v-text-field>
      <v-btn :disabled="!valid" color="success" class="mr-4" @click="submit">Submit</v-btn>
    </v-form>
  </v-row>
</template>

<script lang="ts">
import Vue from "vue";

// @ is an alias to /src
export default Vue.extend({
  name: "Login",
  components: {
    // HelloWorld
  },
  data: () => ({
    valid: true,
    name: "",
    nameRules: [
      (v: string) => !!v || "Name is required",
      (v: string) => (v && v.length <= 10) || "Name must be less than 10 characters"
    ],
    email: "",
    emailRules: [
      (v: string) => !!v || "E-mail is required",
      (v: string) => /.+@.+\..+/.test(v) || "E-mail must be valid"
    ],
  }),
  computed:{
  // In component, import `VForm`
    form() :any {
      return this.$refs.form;
    }
  },
  methods: {
    submit(){
      console.log("Submit")
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