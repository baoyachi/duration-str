<script setup>
import playground from './components/Playground.vue'
</script>

<template>
  <div id="app" class="app-container">
    <playground/>
  </div>
</template>

<style>
body, html {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

.app-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh; /* 确保有足够的高度以居中显示 */
  width: 100%;
  background: linear-gradient(
      135deg, /* 渐变角度，可以根据喜好调整 */
      #0a0f14, /* 深色起始，接近黑色，代表夜幕下的城市 *
      #23395b, /* 稍浅的蓝紫色，模拟远处微光 */
      #2c538b, /* 更明亮的蓝，代表霓虹开始显现 */
      #586960 /* 最亮的部分，霓虹的闪耀效果 */
  );
}

</style>
