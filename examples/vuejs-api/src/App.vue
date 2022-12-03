<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  data() {
    return {
      info: {},
    };
  },
  async mounted() {
    if (!this.info) {
      this.info = await this.load();
    }
  },
  methods: {
    async load() {
      try {
        const result = await fetch("https://awesomeapi.com/version");
        return await result.text();
      } catch (e: any) {
        return `Error: ${e.message}`;
      }
    },
    async serverPrefetch() {
      try {
        this.info = await this.load();
      } catch (e: any) {
        this.info = `Error: ${e.message}`;
      }
    },
  },
});
</script>

<template>
  <div>{{ info }}</div>
</template>
