import {createSSRApp} from "vue";
import {renderToString} from "@vue/server-renderer";

import App from "./Sidebar.vue";
export async function render() {
    const app = createSSRApp(App);

    const html = await renderToString(app);
    return `${html}`;
}
