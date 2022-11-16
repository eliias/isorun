export async function render(bundlePath) {
  return import(bundlePath).then(module => module.render());
}
