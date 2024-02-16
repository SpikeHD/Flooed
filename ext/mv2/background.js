// csp be gon
chrome.webRequest.onHeadersReceived.addListener(
  ({ responseHeaders }) => {
    responseHeaders = responseHeaders.filter((header) => header.name.toLowerCase() !== "content-security-policy");

    return { responseHeaders };
  },

  { urls: ["*://*.discord.com/*", "*://discord.com/*"] },
  ["blocking", "responseHeaders"],
);

chrome.runtime.onInstalled.addListener(() => {
  chrome.tabs.query({ url: "*://discord.com/*" }, (tabs) => {
    // Reload the page to apply the new CSP
    tabs.forEach((tab) => chrome.tabs.reload(tab.id));
  });
});