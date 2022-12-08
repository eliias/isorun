import {ApolloClient, InMemoryCache, HttpLink, HttpOptions} from "@apollo/client/core";
import {apollo} from "@isorun/rails";

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
        cache.restore(state["default"]);
    }

    const httpLinkOptions: HttpOptions = {
        uri: "http://localhost:3000/graphql",
    };
    if (isSSR) {
        httpLinkOptions.fetch = apollo.fetch;
    }

    return new ApolloClient({
        ssrMode: isSSR,
        cache,
        link: new HttpLink(httpLinkOptions),
        ssrForceFetchDelay: isSSR ? 0 : 100
    });
}
