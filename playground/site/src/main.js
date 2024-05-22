import {createApp} from 'vue';
import App from './App';

const app = createApp(App);

app.config.productionTip = false;

app.mixin({
    async beforeCreate() {
        this.wasmLib = await import('duration-wasm');
    },
});

app.mount('#app');
