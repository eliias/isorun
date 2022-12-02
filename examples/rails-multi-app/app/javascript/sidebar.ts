import {createSSRApp} from "vue";

import App from "./Sidebar.vue";

const app = createSSRApp(App);

app.mount("#sidebar");
