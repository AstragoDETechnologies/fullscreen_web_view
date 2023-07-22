<template>
  <main>
    <iframe
      v-if="config"
      class="h-screen w-screen"
      width="100%"
      height="100%"
      :src="
        'https://www.youtube.com/embed/' +
        config.video_id +
        '?start=' +
        config.start_time +
        '&autoplay=1'
      "
      title="YouTube video player"
      frameborder="0"
      allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
      allowfullscreen
    ></iframe>
  </main>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api";

let config = ref<Config | null>(null);

interface Config {
  video_id: String;
  start_time: number;
}

async function load_config() {
  let loaded_config: Config = await invoke("load_config");
  config.value = loaded_config;
}

onMounted(async () => {
  load_config();
});
</script>
