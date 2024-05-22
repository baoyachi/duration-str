<template>

  <div class="terminal-container">
    <div class="terminal-title">Duration-str playground</div>
    <div class="terminal" @click="focusInput">
      <div ref="output" class="output" v-html="outputText"></div>
      <div class="prompt">
        <span class="prompt-start">&gt;</span>
        <input ref="cmdInput" class="cmd-input" type="text" placeholder=" Enter your string duration..."
               @keydown.enter.prevent="onEnter">
      </div>
    </div>
    <div class="corner-text">
      <div class="corner-link" @click="openInNewTab" style="cursor:pointer;">www.github.com/baoyachi/duration-str</div>
    </div>
  </div>
</template>


<script>
import {ref} from 'vue';

export default {
  components: {},
  data() {
    return {
      outputText: '',
      cmdInput: ref(null),
    };
  },
  methods: {
    appendToOutput(text) {
      this.outputText += text;
    },
    focusInput() {
      this.$refs.cmdInput.focus();
    },
    onEnter() {
      const command = this.$refs.cmdInput.value.trim();
      if (command) {
        this.appendToOutput(`> ${command}\n`);
        this.processCommand(command);
        this.$refs.cmdInput.value = ''; // 清空输入框
      }
    },
    openInNewTab() {
      window.open('https://www.github.com/baoyachi/duration-str', '_blank');
    },
    scrollToBottom() {
      this.$refs.cmdInput.value.scrollIntoView({behavior: 'smooth'});
    },
    processCommand(command) {
      switch (command.toLowerCase()) {
        case 'help':
          this.appendToOutput('Available commands: help, version, [duration-str]' + '\n');
          break;
        case 'version':
          const version = this.wasmLib.version();
          this.appendToOutput(version + '\n');
          break;
        default:
          const result = this.wasmLib.parse(command);
          if (result.has("ok")) {
            const okValue = result.get("ok");
            this.appendToOutput(okValue.toString() + '\n');
          } else {
            const errValue = result.get("err");
            this.appendToOutput(errValue.toString() + '\n');
          }
      }
    }
  }
}
</script>


<style>
.terminal-container {
  display: flex; /* 使用Flexbox布局 */
  flex-direction: column; /* 默认方向是从上到下排列子元素 */
  align-items: center; /* 水平居中对齐子元素 */
}

.corner-text {
  display: flex;
  align-items: center;
  line-height: 1;
  margin-top: 10px;
  font-weight: bold;
  right: 10px; /* 距离右侧的距离，可以根据需要调整 */
  color: #fff; /* 文字颜色，与终端内其他文字保持一致或根据需要调整 */
  font-family: 'Courier New', monospace; /* 字体，保持与终端内字体一致 */
  font-size: 14px; /* 文字大小，根据需要调整 */
}

.corner-text svg,
.corner-text a {
  vertical-align: middle;
}

.corner-link {
  margin-left: 6px;
  font-weight: bold;
  color: #fff;
  text-decoration: underline;
  text-decoration-color: currentColor;
}

.terminal-title {
  /* 移除绝对定位相关属性 */
  padding: 10px;
  color: #FF00FF;
  font-family: 'Arial Black', Gadget, sans-serif;
  font-size: 28px;
  font-weight: bold;
  text-shadow: 0 0 5px #000000;
  background-color: #00000080;
  backdrop-filter: blur(5px);
  width: fit-content;
  text-align: center;
  /* 可以根据需要添加一些外边距来与其他元素隔开 */
  margin-bottom: 10px;
}

.terminal {
  min-height: 400px; /* 或者具体的高度比例，例如50% */
  width: 800px; /* 使宽度充满其父容器 */
  background-color: #000;
  border-radius: 5px;
  padding: 10px;
  box-shadow: inset 0 0 10px #33ff99;
  display: flex;
  flex-direction: column;
  /* 当内容溢出时自动显示滚动条 */
  overflow-y: auto;
  /* 保持水平居中 */
  margin: 0 auto;
}

.prompt {
  display: flex;
  align-items: center; /* 使内容垂直居中对齐 */
}

.cmd-input {
  margin-left: 5px; /* 根据需要调整与文本的间距 */
  flex-grow: 1;
  border: none;
  background-color: transparent;
  color: #fff;
  font-family: 'Courier New', monospace;
  font-size: 16px;
  outline: none;
  border-radius: 0;
}

.output {
  margin-top: 8px;
  white-space: pre-wrap;
  color: #33ff99;
  font-family: 'Courier New', monospace;
  max-width: 100%; /* 或者给一个固定的像素值，例如: 800px */
  word-break: break-all; /* 确保长单词也能换行 */
  overflow-y: auto;
}

.prompt-start {
  color: #33ff99;
  font-style: italic;
  align-items: center;
}

</style>