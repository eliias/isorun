import * as React from "react";
import {getDataFromTree} from "@apollo/client/react/ssr";
import {fetch as rubyFetch} from "@isorun/rails";

import {App} from "../my_app/App.jsx";
import {ApolloClient, ApolloProvider, HttpLink, InMemoryCache} from "@apollo/client";
import {renderToStaticMarkup} from "react-dom/server";

function createClient(isSSR) {
  return new ApolloClient({
    ssrMode: isSSR,
    cache: new InMemoryCache(),
    link: new HttpLink({
      uri: 'http://localhost:3000/graphql',
      fetch: rubyFetch
    })
  });
}

const Script = ({state}) => {
  return (
    <script dangerouslySetInnerHTML={{
      __html: `window.__APOLLO_STATE__=${JSON.stringify(state).replace(/</g, '\\u003c')};`,
    }}/>
  );
}

export async function render() {
  const client = createClient(true);

  const Wrapper = (
    <ApolloProvider client={client}>
      <App/>
    </ApolloProvider>
  );

  return getDataFromTree(Wrapper)
    .then((content) => {
      const state = client.extract();
      return content + "\n" + renderToStaticMarkup(<Script state={state}/>);
    });
}
