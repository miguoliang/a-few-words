import { Storage } from "@plasmohq/storage"

export {}

const storage = new Storage()

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
        func: () => {
          console.log('aaa')
          return window.getSelection()?.toString()
        }
      },
      async (selectedText) => {
        console.log('bbb')
        const text = selectedText[0].result
        await storage.set("text", text)
      }
    )
  }
})
