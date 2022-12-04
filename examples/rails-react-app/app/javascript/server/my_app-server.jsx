import * as React from "react";
import {getDataFromTree} from "@apollo/client/react/ssr";
import {ApolloClient, ApolloProvider, HttpLink, InMemoryCache} from "@apollo/client";
import {renderToStaticMarkup} from "react-dom/server";
import {apollo} from "@isorun/rails";

import {App} from "../my_app/App.jsx";

function createClient(isSSR) {
  return new ApolloClient({
    ssrMode: isSSR,
    cache: new InMemoryCache(),
    link: new HttpLink({
      uri: 'http://localhost:3000/graphql',
      fetch: apollo.fetch
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

export default async function render() {
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
