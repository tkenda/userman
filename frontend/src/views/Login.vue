<template>
  <v-layout>
    <v-main>
      <v-sheet class="bg-grey pa-16 sheet-background overflow-hidden" height="100vh">
        <LoginCard class="login" />
      </v-sheet>
      <Footer class="sticky-footer" />
    </v-main>
  </v-layout>
</template>

<script lang="ts" setup>
import LoginCard from "../components/LoginCard.vue";
import Footer from "../components/Footer.vue";
</script>

<style scoped>
.login {
  z-index: 1;
}
.sticky-footer {
  position: fixed;
  bottom: 0;
  width: 100vw;
}

.sheet-background {
  background: #061d53; /* Old browsers */
  background: -moz-linear-gradient(
    -45deg,
    #016060 0%,
    #061d53 100%
  ); /* FF3.6+ */
  background: -webkit-gradient(
    linear,
    left top,
    right bottom,
    color-stop(0%, #016060),
    color-stop(100%, #061d53)
  ); /* Chrome,Safari4+ */
  background: -webkit-linear-gradient(
    -45deg,
    #016060 0%,
    #061d53 100%
  ); /* Chrome10+,Safari5.1+ */
  background: -o-linear-gradient(
    -45deg,
    #016060 0%,
    #061d53 100%
  ); /* Opera 11.10+ */
  background: -ms-linear-gradient(-45deg, #016060 0%, #061d53 100%); /* IE10+ */
  background: linear-gradient(135deg, #016060 0%, #061d53 100%); /* W3C */
  filter: progid:DXImageTransform.Microsoft.gradient( startColorstr='#EA5C54 ', endColorstr='#bb6dec',GradientType=1 ); /* IE6-9 fallback on horizontal gradient */
}
</style>
