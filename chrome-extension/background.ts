import { createWord, AUTH_HOST, type Word, AUTH_CLIENT_ID } from "~content"

export {}

chrome.runtime.onInstalled.addListener(async () => {
  chrome.contextMenus.create({
    id: "getSelectedText",
    title: "Save to A few words",
    contexts: ["selection"]
  })
  chrome.sidePanel.setPanelBehavior({ openPanelOnActionClick: true })
})

// Open a new search tab when the user clicks a context menu
chrome.contextMenus.onClicked.addListener((item, tab) => {
  const tld = item.menuItemId
  if (tld === "getSelectedText") {
    chrome.scripting.executeScript(
      {
        target: { tabId: tab.id },
        func: () => window.getSelection()?.toString()
      },
      async (selectedText) => {
        const text = selectedText[0].result
        if (!text) return
        const word: Word = { word: text.trim() }
        await createWord(word)
      }
    )
  }
})

chrome.runtime.onMessage.addListener(function(request) {
  if (request.action === 'openNewTab') {
    const searchParams = new URLSearchParams()
    searchParams.append("client_id", AUTH_CLIENT_ID)
    searchParams.append("response_type", "code")
    searchParams.append("redirect_uri", chrome.identity.getRedirectURL())
    searchParams.append("scope", "openid email profile")
    const url = `${AUTH_HOST}/signup?${searchParams.toString()}`;
    chrome.tabs.create({ url });
  }
});