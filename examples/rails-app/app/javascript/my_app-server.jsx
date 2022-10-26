import * as React from "react";
import * as Server from "react-dom/server";

import {App} from "./my_app/App.jsx";

export function render() {
    return Promise.resolve(Server.renderToString(<App/>));
}
