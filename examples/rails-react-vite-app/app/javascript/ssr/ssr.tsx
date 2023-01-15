import {renderToString} from "react-dom/server";

export default async function() {
  return renderToString(<h1>Hello, world!</h1>);
}
