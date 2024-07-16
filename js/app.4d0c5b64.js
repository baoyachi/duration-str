(function(){"use strict";var n={714:function(n,t,e){var r=e(9670),o=e(3151);const i={class:"terminal-container"},u=(0,o._)("div",{class:"terminal-title"},"Duration-str playground",-1),a=["innerHTML"],c={class:"prompt"},s=(0,o._)("span",{class:"prompt-start"},">",-1),f={class:"corner-text"},p=(0,o._)("svg",{t:"1716403568580",class:"icon",viewBox:"0 0 1024 1024",version:"1.1",xmlns:"http://www.w3.org/2000/svg","p-id":"1734",width:"16",height:"16"},[(0,o._)("path",{d:"M512 12.64c-282.752 0-512 229.216-512 512 0 226.208 146.72 418.144 350.144 485.824 25.6 4.736 35.008-11.104 35.008-24.64 0-12.192-0.48-52.544-0.704-95.328-142.464 30.976-172.512-60.416-172.512-60.416-23.296-59.168-56.832-74.912-56.832-74.912-46.464-31.776 3.52-31.136 3.52-31.136 51.392 3.616 78.464 52.768 78.464 52.768 45.664 78.272 119.776 55.648 148.992 42.56 4.576-33.088 17.856-55.68 32.512-68.48-113.728-12.928-233.28-56.864-233.28-253.024 0-55.904 20-101.568 52.768-137.44-5.312-12.896-22.848-64.96 4.96-135.488 0 0 43.008-13.76 140.832 52.48 40.832-11.36 84.64-17.024 128.16-17.248 43.488 0.192 87.328 5.888 128.256 17.248 97.728-66.24 140.64-52.48 140.64-52.48 27.872 70.528 10.336 122.592 5.024 135.488 32.832 35.84 52.704 81.536 52.704 137.44 0 196.64-119.776 239.936-233.792 252.64 18.368 15.904 34.72 47.04 34.72 94.816 0 68.512-0.608 123.648-0.608 140.512 0 13.632 9.216 29.6 35.168 24.576 203.328-67.776 349.856-259.616 349.856-485.76 0-282.784-229.248-512-512-512z",fill:"#ffffff","p-id":"1735"})],-1);function l(n,t,e,l,d,m){return(0,o.wg)(),(0,o.iD)("div",i,[u,(0,o._)("div",{class:"terminal",onClick:t[1]||(t[1]=(...n)=>m.focusInput&&m.focusInput(...n))},[(0,o._)("div",{ref:"output",class:"output",innerHTML:d.outputText},null,8,a),(0,o._)("div",c,[s,(0,o._)("input",{ref:"cmdInput",class:"cmd-input",type:"text",placeholder:" Enter your string duration...",onKeydown:t[0]||(t[0]=(0,r.D2)((0,r.iM)(((...n)=>m.onEnter&&m.onEnter(...n)),["prevent"]),["enter"]))},null,544)])]),(0,o._)("div",f,[p,(0,o._)("div",{class:"corner-link",onClick:t[2]||(t[2]=(...n)=>m.openInNewTab&&m.openInNewTab(...n)),style:{cursor:"pointer"}},"www.github.com/baoyachi/duration-str")])])}var d=e(2318),m={components:{},data(){return{outputText:"",cmdInput:(0,d.iH)(null)}},methods:{appendToOutput(n){this.outputText+=n},focusInput(){this.$refs.cmdInput.focus()},onEnter(){const n=this.$refs.cmdInput.value.trim();n&&(this.appendToOutput(`> ${n}\n`),this.processCommand(n),this.$refs.cmdInput.value="")},openInNewTab(){window.open("https://www.github.com/baoyachi/duration-str","_blank")},scrollToBottom(){this.$refs.cmdInput.value.scrollIntoView({behavior:"smooth"})},processCommand(n){switch(n.toLowerCase()){case"help":this.appendToOutput("Available commands: help, version, [duration-str]\n");break;case"version":const t=this.wasmLib.version();this.appendToOutput(t+"\n");break;default:const e=this.wasmLib.parse(n);if(e.has("ok")){const n=e.get("ok");this.appendToOutput(n.toString()+"\n")}else{const n=e.get("err");this.appendToOutput(n.toString()+"\n")}}}}},h=e(5052);const v=(0,h.Z)(m,[["render",l]]);var b=v;const w={id:"app",class:"app-container"};var y={__name:"App",setup(n){return(n,t)=>((0,o.wg)(),(0,o.iD)("div",w,[(0,o.Wm)(b)]))}};const g=y;var _=g;const k=(0,r.ri)(_);k.config.productionTip=!1,k.mixin({async beforeCreate(){this.wasmLib=await e.e(827).then(e.bind(e,827))}}),k.mount("#app")}},t={};function e(r){var o=t[r];if(void 0!==o)return o.exports;var i=t[r]={id:r,exports:{}};return n[r].call(i.exports,i,i.exports,e),i.exports}e.m=n,function(){var n="function"===typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",t="function"===typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",r="function"===typeof Symbol?Symbol("webpack error"):"__webpack_error__",o=function(n){n&&n.d<1&&(n.d=1,n.forEach((function(n){n.r--})),n.forEach((function(n){n.r--?n.r++:n()})))},i=function(e){return e.map((function(e){if(null!==e&&"object"===typeof e){if(e[n])return e;if(e.then){var i=[];i.d=0,e.then((function(n){u[t]=n,o(i)}),(function(n){u[r]=n,o(i)}));var u={};return u[n]=function(n){n(i)},u}}var a={};return a[n]=function(){},a[t]=e,a}))};e.a=function(e,u,a){var c;a&&((c=[]).d=-1);var s,f,p,l=new Set,d=e.exports,m=new Promise((function(n,t){p=t,f=n}));m[t]=d,m[n]=function(n){c&&n(c),l.forEach(n),m["catch"]((function(){}))},e.exports=m,u((function(e){var o;s=i(e);var u=function(){return s.map((function(n){if(n[r])throw n[r];return n[t]}))},a=new Promise((function(t){o=function(){t(u)},o.r=0;var e=function(n){n!==c&&!l.has(n)&&(l.add(n),n&&!n.d&&(o.r++,n.push(o)))};s.map((function(t){t[n](e)}))}));return o.r?a:u()}),(function(n){n?p(m[r]=n):f(d),o(c)})),c&&c.d<0&&(c.d=0)}}(),function(){var n=[];e.O=function(t,r,o,i){if(!r){var u=1/0;for(f=0;f<n.length;f++){r=n[f][0],o=n[f][1],i=n[f][2];for(var a=!0,c=0;c<r.length;c++)(!1&i||u>=i)&&Object.keys(e.O).every((function(n){return e.O[n](r[c])}))?r.splice(c--,1):(a=!1,i<u&&(u=i));if(a){n.splice(f--,1);var s=o();void 0!==s&&(t=s)}}return t}i=i||0;for(var f=n.length;f>0&&n[f-1][2]>i;f--)n[f]=n[f-1];n[f]=[r,o,i]}}(),function(){e.n=function(n){var t=n&&n.__esModule?function(){return n["default"]}:function(){return n};return e.d(t,{a:t}),t}}(),function(){e.d=function(n,t){for(var r in t)e.o(t,r)&&!e.o(n,r)&&Object.defineProperty(n,r,{enumerable:!0,get:t[r]})}}(),function(){e.f={},e.e=function(n){return Promise.all(Object.keys(e.f).reduce((function(t,r){return e.f[r](n,t),t}),[]))}}(),function(){e.u=function(n){return"js/"+n+".46d9c0f9.js"}}(),function(){e.miniCssF=function(n){}}(),function(){e.g=function(){if("object"===typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(n){if("object"===typeof window)return window}}()}(),function(){e.o=function(n,t){return Object.prototype.hasOwnProperty.call(n,t)}}(),function(){var n={},t="duration-str-playground:";e.l=function(r,o,i,u){if(n[r])n[r].push(o);else{var a,c;if(void 0!==i)for(var s=document.getElementsByTagName("script"),f=0;f<s.length;f++){var p=s[f];if(p.getAttribute("src")==r||p.getAttribute("data-webpack")==t+i){a=p;break}}a||(c=!0,a=document.createElement("script"),a.charset="utf-8",a.timeout=120,e.nc&&a.setAttribute("nonce",e.nc),a.setAttribute("data-webpack",t+i),a.src=r),n[r]=[o];var l=function(t,e){a.onerror=a.onload=null,clearTimeout(d);var o=n[r];if(delete n[r],a.parentNode&&a.parentNode.removeChild(a),o&&o.forEach((function(n){return n(e)})),t)return t(e)},d=setTimeout(l.bind(null,void 0,{type:"timeout",target:a}),12e4);a.onerror=l.bind(null,a.onerror),a.onload=l.bind(null,a.onload),c&&document.head.appendChild(a)}}}(),function(){e.r=function(n){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(n,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(n,"__esModule",{value:!0})}}(),function(){e.v=function(n,t,r,o){var i=fetch(e.p+""+r+".module.wasm");return"function"===typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(i,o).then((function(t){return Object.assign(n,t.instance.exports)})):i.then((function(n){return n.arrayBuffer()})).then((function(n){return WebAssembly.instantiate(n,o)})).then((function(t){return Object.assign(n,t.instance.exports)}))}}(),function(){e.p=""}(),function(){var n={143:0};e.f.j=function(t,r){var o=e.o(n,t)?n[t]:void 0;if(0!==o)if(o)r.push(o[2]);else{var i=new Promise((function(e,r){o=n[t]=[e,r]}));r.push(o[2]=i);var u=e.p+e.u(t),a=new Error,c=function(r){if(e.o(n,t)&&(o=n[t],0!==o&&(n[t]=void 0),o)){var i=r&&("load"===r.type?"missing":r.type),u=r&&r.target&&r.target.src;a.message="Loading chunk "+t+" failed.\n("+i+": "+u+")",a.name="ChunkLoadError",a.type=i,a.request=u,o[1](a)}};e.l(u,c,"chunk-"+t,t)}},e.O.j=function(t){return 0===n[t]};var t=function(t,r){var o,i,u=r[0],a=r[1],c=r[2],s=0;if(u.some((function(t){return 0!==n[t]}))){for(o in a)e.o(a,o)&&(e.m[o]=a[o]);if(c)var f=c(e)}for(t&&t(r);s<u.length;s++)i=u[s],e.o(n,i)&&n[i]&&n[i][0](),n[i]=0;return e.O(f)},r=self["webpackChunkduration_str_playground"]=self["webpackChunkduration_str_playground"]||[];r.forEach(t.bind(null,0)),r.push=t.bind(null,r.push.bind(r))}();var r=e.O(void 0,[998],(function(){return e(714)}));r=e.O(r)})();
//# sourceMappingURL=app.4d0c5b64.js.map