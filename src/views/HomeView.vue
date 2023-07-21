<template>
  <main>
    <iframe
      class="h-screen w-screen"
      width="100%"
      height="100%"
      :src="'https://www.youtube.com/embed/' + video_id + '?autoplay=1'"
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

async function load_video() {
  video_id.value = await invoke("load_video_id");
}

onMounted(async () => {
  await load_video();
});
</script>
