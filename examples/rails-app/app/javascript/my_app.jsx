import * as React from "react";
import {hydrateRoot} from "react-dom/client";

import {App} from "./my_app/App.jsx";

const container = document.querySelector('#my_app');
hydrateRoot(container, <App/>);
