class FlooedApi {
  ws = null;
  version = null;
  commandsWaiting = {};
  commandIdx = 0;
  shouldShowUnreadBadge = false;

  util = {
    cssSanitize: (css) => {
      const style = document.createElement('style')
      style.innerHTML = css
    
      document.head.appendChild(style)
    
      if (!style.sheet) return
    
      const result = Array.from(style.sheet.cssRules).map(rule => rule.cssText || '').join('\n')
    
      document.head.removeChild(style)
      return result
    },
    fetchImage: async function (url) {
      // Flooed uses a web extension that removes CORs, so it's fine
      return (await fetch(url)).blob();
    },
    applyNotificationCount: () => {}
  }

  constructor() {
    this.ws = new WebSocket("ws://127.0.0.1:10102");

    this.ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        console.log('[Flooed] Received message: ', data)
  
        if (data.command === 'response') {
          if (this.commandsWaiting[data.id]) {
            this.commandsWaiting[data.id](data.data);
            delete this.commandsWaiting[data.id];
          }
        }
      } catch (e) {
        console.error('[Flooed] Command failed')
      }
    }
  }

  async invoke(command, data) {
    // Ensure we wait for WS to be open
    while (this.ws.readyState !== WebSocket.OPEN) {
      console.log('[Flooed] Waiting for WS to be open...')
      await new Promise(setTimeout);
    }

    const idx = this.commandIdx++

    console.log('[Flooed] Sending command: ', command)
    
    this.ws.send(JSON.stringify({ command, data, id: idx  }));

    // Wait for response
    return await new Promise((resolve) => {
      this.commandsWaiting[idx] = resolve;
    });
  }
}

;(async () => {
  // Recreate localStorage
  console.log('[Create LocalStorage] Injecting...')
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

  console.log('[Flooed] Injecting...')
  // Then we define window.Flooed
  scriptTagExec(`
  ${FlooedApi.toString()}

  window.Flooed = new FlooedApi()

  ;(async () => {
    console.log('[Flooed] Fetching version...')
    window.Flooed.version = await Flooed.invoke('get_version')
    console.log('[Flooed] Version: ', window.Flooed.version)
  })()
  `)

  await ensurePlugins();
})()

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