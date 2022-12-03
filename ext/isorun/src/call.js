export async function call(bundlePath, entrypoint, args, kwargs) {
  return import(bundlePath).then(module => module[entrypoint].call(undefined, ...args, kwargs));
}
