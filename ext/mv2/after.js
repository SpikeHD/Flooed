;(async () => {
  // Recreate localStorage
  scriptTagExec(`
  const iframe = document.createElement('iframe')

  // Wait for document.head to exist, then append the iframe
  const interval = setInterval(() => {
    if (!document.head || window.localStorage) return

    document.head.append(iframe)
    const pd = Object.getOwnPropertyDescriptor(iframe.contentWindow, 'localStorage')
    iframe.remove()

    if (!pd) return

    Object.defineProperty(window, 'localStorage', pd)

    console.log('[Create LocalStorage] Done!')

    clearInterval(interval)
  }, 50)
  `)

  const webuiScript = await fetch("http://localhost:10100/webui.js")

  // WebUI goes first
  scriptTagExec(await webuiScript.text())

  dispatchEvent(new Event('load'));

  // Then we define window.Flooed
  scriptTagExec(`
  ${FlooedApi.toString()}

  window.Flooed = new FlooedApi()
  `)

  await ensurePlugins();
})()

class FlooedApi {
  ws = null;
  shouldShowUnreadBadge = false;
  util = {
    cssSanitize: (css) => {},
    fetchImage: async function (url) {
      // Flooed uses a web extension that removes CORs, so it's fine
      return (await fetch(url)).blob();
    },
    applyNotificationCount: () => {}
  }

  constructor() {
    this.ws = new WebSocket("ws://localhost:10102");
  }

  async invoke(command, data) {
    this.ws.send(JSON.stringify({ command, data }));
  }
}

async function ensurePlugins() {
  const requiredPlugins = {
    'Dorion Settings':
      'https://spikehd.github.io/shelter-plugins/dorion-settings/',
    'Dorion Notifications':
      'https://spikehd.github.io/shelter-plugins/dorion-notifications/',
    'Dorion Streamer Mode':
      'https://spikehd.github.io/shelter-plugins/dorion-streamer-mode/',
    'Dorion Updater':
      'https://spikehd.github.io/shelter-plugins/dorion-updater/',
    'Dorion PTT': 'https://spikehd.github.io/shelter-plugins/dorion-ptt/',
    'Dorion Tray': 'https://spikehd.github.io/shelter-plugins/dorion-tray/',
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
        //plugin.onLoad?.()
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