import {send} from "../deno";

export async function fetch(url: string, init?: RequestInit) {
  const args = {url, options: JSON.stringify(init)};
  try {
    // forward request to Ruby
    const raw = await send({action: "fetch", args});
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
