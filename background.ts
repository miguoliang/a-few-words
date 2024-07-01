import { Storage } from "@plasmohq/storage"
import type { CardList } from "~content"

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
        const storage = new Storage()
        const cards = await storage.get<CardList>("cards") || []
        cards.push({ front: text, back: "" })
        await storage.set("cards", cards)
      }
    )
  }
})
