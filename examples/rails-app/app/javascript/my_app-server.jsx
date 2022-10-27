import * as React from "react";
import {renderToStringWithData} from "@apollo/client/react/ssr";

import {App} from "./my_app/App.jsx";

export function render() {
  return renderToStringWithData(<App isSSR={true}/>)
    .then(result => result)
    .catch(err => console.error(err));
}
