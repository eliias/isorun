import pkg from "./../../package.json" assert { type: "json" };

export const VERSION = pkg.version;

export async function send(action: string, args: scalar) {
  return await Deno.core.ops.op_app_send(action, args);
}
