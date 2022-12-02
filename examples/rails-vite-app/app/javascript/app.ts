import {createSSRApp, h, provide} from "vue";
import {DefaultApolloClient} from "@vue/apollo-composable";

import App from "./App.vue";
import {createClient} from "./client";

const client = createClient(false);

const app = createSSRApp({
    setup() {
        provide(DefaultApolloClient, client);
    },
    render: () => h(App),
});

app.mount("#app");
