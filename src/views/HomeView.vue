<template>
  <main>
    <iframe
      class="h-screen w-screen"
      width="100%"
      height="100%"
      :src="
        'https://www.youtube.com/embed/' +
        video_id +
        '?start=' +
        start_time +
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

let video_id = ref("");
let start_time = ref(0);

async function load_video() {
  video_id.value = await invoke("load_video_id");
}

async function load_start_time() {
  start_time.value = await invoke("load_start_time");
}

onMounted(async () => {
  await load_video();
  await load_start_time();
});
</script>
