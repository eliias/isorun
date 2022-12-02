import {createSSRApp, h, provide} from "vue";
import {renderToString} from "@vue/server-renderer";
import {getStates} from "@vue/apollo-ssr";
import {DefaultApolloClient} from "@vue/apollo-composable";

import App from "./App.vue";
import {createClient} from "./client";


type Args = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  state: Record<string, Record<string, any>>;
}

const renderScript = ({state}: Args): string => {
    return `
    <script>
        window.__APOLLO_STATE__=${JSON.stringify(state).replace(/</g, "\\u003c")};
    </script>`;
};

export async function render() {
    const client = createClient(true);

    const app = createSSRApp({
        setup() {
            provide(DefaultApolloClient, client);
        },
        render: () => h(App),
    });

    const html = await renderToString(app);
    const args = {state: getStates({"default": client})};
    const script = renderScript(args);

    return `${html}\n${script}`;
}
