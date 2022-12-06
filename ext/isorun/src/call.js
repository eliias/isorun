export async function call(options, args, kwargs) {
  const {bundle_path: bundlePath, entrypoint, force_reload: forceReload} = options;

  // Filter options that should not be passed to JavaScript
  ["bundle_path", "entrypoint", "force_reload", "message_receiver"].forEach(i => delete options[i]);

  try {
    let fullBundleUrl = bundlePath
    if (forceReload) {
      fullBundleUrl += `?t=${Date.now()}`
      console.info("[ISORUN] Forced loading bundle");
    }

    const module = await import(fullBundleUrl);
    const callee = module[entrypoint];

    // check if callee is actually a function, otherwise just return the member
    // as is
    if (typeof callee === "function") {
      return await callee.call(undefined, ...args, kwargs);
    } else {
      return callee;
    }
  } catch (err) {
    throw new Error(`failed to call "${entrypoint}" in "${bundlePath}" with options: ${JSON.stringify(options, null, "  ")}`);
  }
}

export async function load(url) {
  try {
    return await import(url);
  } catch(err) {
    throw new Error(`failed to import module: "${url}"`);
  }
}
