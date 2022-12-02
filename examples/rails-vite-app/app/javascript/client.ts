import {ApolloClient, InMemoryCache, HttpLink, HttpOptions} from "@apollo/client/core";

declare global {
  interface Window {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    __APOLLO_STATE__: any;
  }
}

export function createClient(isSSR = false) {
    const state = window.__APOLLO_STATE__ || null;
    const cache = new InMemoryCache();
    if (state != null) {
        cache.restore(state);
    }

    const httpLinkOptions: HttpOptions = {
        uri: "http://localhost:3000/graphql",
    };
    if (isSSR) {
        httpLinkOptions.fetch = async (url, options) => {
            const args = JSON.stringify({url, options});
            try {
                // forward request to Ruby
                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-ignore
                const raw = await Deno.core.ops.op_app_send("fetch", args);
                if (raw !== "") {
                    return new Response(raw);
                }

                // we failed to get data, reject promise (fail)
                return Promise.reject("failed to fetch data, stopping now");
            } catch (err) {
                console.error(err);
            }
            return new Response("");
        };
    }

    return new ApolloClient({
        ssrMode: isSSR,
        cache,
        link: new HttpLink(httpLinkOptions),
        ssrForceFetchDelay: isSSR ? 100 : 0
    });
}
