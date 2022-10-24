import {createSSRApp} from "vue"
import {renderToString} from "vue/server-renderer"
import {createPinia} from 'pinia'


import App from './App.vue'
import router from './router'

import './assets/main.css'

const app = createSSRApp(App)

app.use(createPinia())
app.use(router)

export function render() {
    return renderToString(app).then((html: string) => {
        return `
            <!DOCTYPE html>
            <html lang="en">
              <head>
                <title>Vue SSR Example</title>
              </head>
              <body>
                <div id="app">${html}</div>
              </body>
            </html>`;
    });
}
