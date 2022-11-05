export async function render() {
  await Deno.core.ops.op_app_send("Bam, oida!");
  return Promise.resolve(`<h1>Hello, World!</h1>`);
}
