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
            try {
              // For compat reasons, config must stay a string
              if (!JSON.parse(data.data).client_type) {
                data.data = JSON.parse(data.data)
              }
            } catch (e) {}

            this.commandsWaiting[data.id](data.data);
            delete this.commandsWaiting[data.id];
          }
        }
      } catch (e) {
        console.error('[Flooed] Command failed: ', e)
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

async function init() {
  window.Flooed = new FlooedApi()

  console.log('[Flooed] Fetching version...')
  window.Flooed.version = await Flooed.invoke('get_version')
  console.log('[Flooed] Version: ', window.Flooed.version)

  console.log('[Client Mod Loader] Loading additional mods...')

  const mods = await Flooed.invoke('load_client_mods_js')
  const modsCss = await Flooed.invoke('load_client_mods_css')

  window.eval(mods)

  // Client mod CSS
  const style = document.createElement('style')
  style.innerHTML = modsCss
  document.head.appendChild(style)

  console.log('[Theme Loader] Loading theme...')

  const config = JSON.parse(await Flooed.invoke('read_config_file'))
  const theme = await Flooed.invoke('get_theme', { name: config.theme })

  console.log('[Theme Loader] Theme: ', config.theme)

  if (theme !== '') {
    const css = window.Flooed.util.cssSanitize(theme)
    const style = document.createElement('style')
    style.innerHTML = css
    style.id = 'flooed-theme'

    document.body.appendChild(style)
  }
}

;(async () => {
  console.log('[Flooed] Injecting...')

  // Then we define window.Flooed
  scriptTagExec(`
  ${FlooedApi.toString()}
  ;(${init.toString()})()
  `)

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

  await ensurePlugins();
})()

async function scriptTagExec(script) {
  const scriptTag = document.createElement("script");
  scriptTag.textContent = script;

  while (!document.head) await new Promise(setTimeout);
  document.head.append(scriptTag);
}