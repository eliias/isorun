export async function render(bundlePath) {
  console.log("Start SSR: ", bundlePath)
  return import(bundlePath).then(module => module.render());
}
