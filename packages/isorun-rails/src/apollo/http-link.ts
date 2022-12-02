import {send} from "../deno";

export async function fetch(url: RequestInfo | URL, init?: RequestInit) {
  const args = JSON.stringify({url, options: init});
  try {
    // forward request to Ruby
    const raw = await send("fetch", args);
    if (raw != null) {
      return new Response(raw.toString());
    }

    // we failed to get data, reject promise (fail)
    return Promise.reject("failed to fetch data, stopping now");
  } catch (err) {
    console.error(err);
  }
  return new Response("");
}
