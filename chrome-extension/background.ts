import { createWord, type Word } from "~content"

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
        const word: Word = { word: text }
        await createWord(word)
      }
    )
  }
})
