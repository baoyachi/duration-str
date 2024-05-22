const {defineConfig} = require('@vue/cli-service')

module.exports = defineConfig({
    transpileDependencies: true,
    publicPath: './',
    configureWebpack: {
        experiments: {
            asyncWebAssembly: true
        },
        plugins: []
    }
})
