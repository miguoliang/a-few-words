import type { Word } from "~content"
import { store } from "~store"

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
        createWord(word)
          .then((res) => {
            if (res.ok) {
              console.log("success")
            } else {
              console.error(res.status)
            }
          })
          .catch((error) => {
            console.error(error)
          })
      }
    )
  }
})

async function createWord(word: Word) {
  return fetch("http://localhost:8000/words", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + store?.getState().auth.access_token
    },
    body: JSON.stringify(word)
  })
}
