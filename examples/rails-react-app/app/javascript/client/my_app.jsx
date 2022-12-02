import * as React from "react";
import {hydrateRoot} from "react-dom/client";
import {ApolloClient, ApolloProvider, HttpLink, InMemoryCache} from "@apollo/client";

import {App} from "../my_app/App.jsx";

function createClient(isSSR) {
  const state = window.__APOLLO_STATE__;
  return new ApolloClient({
    ssrMode: isSSR,
    cache: new InMemoryCache().restore(state),
    link: new HttpLink({
      uri: 'http://localhost:3000/graphql',
    }),
    ssrForceFetchDelay: 100
  });
}

const client = createClient(false);

const Wrapper = (
  <ApolloProvider client={client}>
    <App/>
  </ApolloProvider>
);

const container = document.querySelector('#my_app');
hydrateRoot(container, Wrapper);
