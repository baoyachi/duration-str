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
      <svg t="1716403568580" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="1734" width="16" height="16"><path d="M512 12.64c-282.752 0-512 229.216-512 512 0 226.208 146.72 418.144 350.144 485.824 25.6 4.736 35.008-11.104 35.008-24.64 0-12.192-0.48-52.544-0.704-95.328-142.464 30.976-172.512-60.416-172.512-60.416-23.296-59.168-56.832-74.912-56.832-74.912-46.464-31.776 3.52-31.136 3.52-31.136 51.392 3.616 78.464 52.768 78.464 52.768 45.664 78.272 119.776 55.648 148.992 42.56 4.576-33.088 17.856-55.68 32.512-68.48-113.728-12.928-233.28-56.864-233.28-253.024 0-55.904 20-101.568 52.768-137.44-5.312-12.896-22.848-64.96 4.96-135.488 0 0 43.008-13.76 140.832 52.48 40.832-11.36 84.64-17.024 128.16-17.248 43.488 0.192 87.328 5.888 128.256 17.248 97.728-66.24 140.64-52.48 140.64-52.48 27.872 70.528 10.336 122.592 5.024 135.488 32.832 35.84 52.704 81.536 52.704 137.44 0 196.64-119.776 239.936-233.792 252.64 18.368 15.904 34.72 47.04 34.72 94.816 0 68.512-0.608 123.648-0.608 140.512 0 13.632 9.216 29.6 35.168 24.576 203.328-67.776 349.856-259.616 349.856-485.76 0-282.784-229.248-512-512-512z" fill="#ffffff" p-id="1735"></path></svg>
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