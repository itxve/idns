<script setup lang="ts">
import { repository, version } from "../package.json";
import DNSManager from "@/components/DNSManager.vue";
</script>

<template>
  <div class="container">
    <DNSManager />
    <footer class="footer">
      <div>
        © 2022
        <a target="_blank" :href="repository.url">
          <b>GitHub</b>
        </a>
        V{{ version }}
      </div>
    </footer>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
