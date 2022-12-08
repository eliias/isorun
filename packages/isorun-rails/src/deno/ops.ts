import pkg from "./../../package.json" assert { type: "json" };

export const VERSION = pkg.version;

export async function send(message: {action: string, args: scalar | scalar[] | Record<string, scalar>}) {
  return await Deno.core.ops.op_send_to_ruby(message);
}
