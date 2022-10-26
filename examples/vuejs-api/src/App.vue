<script lang="ts">

export default {
  data() {
    return {
      info: {}
    }
  },
  methods: {
    async load() {
      try {
        const result = await fetch("https://awesomeapi.com/version")
        return await result.text();
      } catch(e) {
        return `Error: ${e.message}`;
      }
    }
  },
  async serverPrefetch() {
    try {
      this.info = await this.load();
    } catch(e) {
      this.info = `Error: ${e.message}`;
    }
  },
  async mounted() {
    if (!this.data) {
      this.info = await this.load();
    }
  }
}
</script>

<template>
  <div>{{ info }}</div>
</template>
