import {createSSRApp} from "vue"
import {renderToString} from "vue/server-renderer"

import App from './App.vue'

const app = createSSRApp(App);

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
