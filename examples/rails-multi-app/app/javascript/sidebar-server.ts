import {createSSRApp} from "vue";
import {renderToString} from "@vue/server-renderer";

import App from "./Sidebar.vue";
export default async function render() {
    const app = createSSRApp(App);

    const html = await renderToString(app);
    return `${html}`;
}
