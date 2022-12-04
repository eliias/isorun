export async function call(options, args, kwargs) {
  const {bundle_path: bundlePath, entrypoint} = options;

  try {
    const module = await import(bundlePath);
    const callee = module[entrypoint];

    // check if callee is actually a function, otherwise just return the member
    // as is
    if (typeof callee === "function") {
      return await callee.call(undefined, ...args, kwargs);
    } else {
      return callee;
    }
  } catch (err) {
    throw new Error(`failed to call "${entrypoint}" in "${bundlePath}" with options: ${options}`);
  }
}
