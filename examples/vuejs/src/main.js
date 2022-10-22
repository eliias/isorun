import {createSSRApp} from 'vue';
import {renderToString} from 'vue/server-renderer';

const app = createSSRApp({
    data: () => ({ count: 1 }),
    template: `<button @click="count++">{{ count }}</button>`
})

renderToString(app).then((html) => {
    const str = `
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <title>Vue SSR Example</title>
      </head>
      <body>
        <div id="app">${html}</div>
      </body>
    </html>`;

    console.log(str);
});
