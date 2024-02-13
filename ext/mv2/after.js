;(async () => {
  const webuiScript = await fetch("http://localhost:10100/webui.js")

  // WebUI goes first
  scriptTagExec(await webuiScript.text())

  // Then we define window.Flooed
  scriptTagExec(`window.Flooed = { }`)

  await ensurePlugins();
})()

async function ensurePlugins() {
  const requiredPlugins = {
    'Dorion Settings':
      'https://spikehd.github.io/shelter-plugins/dorion-settings/',
    'Always Trust': 'https://spikehd.github.io/shelter-plugins/always-trust/',
    'Dorion Notifications':
      'https://spikehd.github.io/shelter-plugins/dorion-notifications/',
    'Dorion Streamer Mode':
      'https://spikehd.github.io/shelter-plugins/dorion-streamer-mode/',
    'Dorion Updater':
      'https://spikehd.github.io/shelter-plugins/dorion-updater/',
    'Dorion PTT': 'https://spikehd.github.io/shelter-plugins/dorion-ptt/',
    'Dorion Tray': 'https://spikehd.github.io/shelter-plugins/dorion-tray/',
    'Dorion Fullscreen':
      'https://spikehd.github.io/shelter-plugins/dorion-fullscreen/',
  }

  const promises = [
    ...Object.entries(requiredPlugins).map(async ([name, url]) => {
      const res = await fetch(`${url}/plugin.js`)
      const text = await res.text()

      // Eval
      try {
        console.log('[Ensure Plugins] Loading plugin: ', name)

        // Create a new plugin object. Simpler version of https://github.com/uwu/shelter/blob/ac74061864479ecb688ae5efc321e981cd1b54fa/packages/shelter/src/plugins.tsx#L54
        const pluginStr = `shelter=>{return ${text}}${atob('Ci8v')}`
        const fn = eval(pluginStr)
        
        scriptTagExec(`(${fn.toString()})(shelter)`)

        // Run plugin.onLoad if it exists
        plugin.onLoad?.()
      } catch (e) {
        console.error(`[Ensure Plugins] Failed to load plugin ${name}: `, e)
      }
    }),
  ]

  await Promise.all(promises)
}

async function scriptTagExec(script) {
  const scriptTag = document.createElement("script");
  scriptTag.textContent = script;

  while (!document.head) await new Promise(setTimeout);
  document.head.append(scriptTag);
}