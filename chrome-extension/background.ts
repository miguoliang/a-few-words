import { setLogout, setTokens } from "~auth-slice"
import {
  createWord,
  OIDC_CLIENT_ID,
  OIDC_HOST,
  translate
} from "~common"
import { store } from "~store"
import { resetWords } from "~words-slice"

export {}

chrome.runtime.onInstalled.addListener(async () => {
  chrome.contextMenus.create({
    id: "getSelectedText",
    title: "Save to A few words",
    contexts: ["selection"]
  })
  chrome.sidePanel.setPanelBehavior({ openPanelOnActionClick: true })
  setInterval(refreshToken, 1000 * 60 * 30)
})

async function refreshToken() {
  const refresh_token = store.getState().auth.refresh_token
  const access_token = store.getState().auth.access_token
  const searchParams = new URLSearchParams()
  searchParams.append("client_id", OIDC_CLIENT_ID)
  searchParams.append("grant_type", "refresh_token")
  searchParams.append("refresh_token", refresh_token)
  if (!access_token) return
  const response = await fetch(`${OIDC_HOST}/oauth2/token`, {
    method: "POST",
    headers: {
      "Content-Type": "application/x-www-form-urlencoded"
    },
    body: searchParams
  })
  if (response.ok) {
    const data = await response.json()
    store.dispatch(setTokens({ ...data, refresh_token }))
  } else {
    store.dispatch(setLogout())
  }
}

// Open a new search tab when the user clicks a context menu
chrome.contextMenus.onClicked.addListener((item, tab) => {
  const tld = item.menuItemId
  if (tld === "getSelectedText") {
    chrome.scripting.executeScript(
      {
        target: { tabId: tab.id },
        func: () => {
          function getPrecedingWords(selection, wordCount) {
            const range = selection.getRangeAt(0)
            let precedingWords = []
            let currentNode = range.startContainer
            let offset = range.startOffset

            while (precedingWords.length < wordCount && currentNode) {
              if (currentNode.nodeType === Node.TEXT_NODE) {
                const text = currentNode.nodeValue.slice(0, offset).split(/\s+/)
                precedingWords = text.concat(precedingWords).slice(-wordCount)
              } else if (
                currentNode.nodeType === Node.ELEMENT_NODE &&
                currentNode.childNodes.length > 0
              ) {
                currentNode =
                  currentNode.childNodes[currentNode.childNodes.length - 1]
                offset =
                  currentNode.nodeType === Node.TEXT_NODE
                    ? currentNode.nodeValue.length
                    : 0
                continue
              }
              offset = currentNode.nodeType === Node.TEXT_NODE ? 0 : null
              currentNode = currentNode.previousSibling
            }

            return precedingWords
          }

          function getFollowingWords(selection, wordCount) {
            const range = selection.getRangeAt(0)
            let followingWords = []
            let currentNode = range.endContainer
            let offset = range.endOffset

            while (followingWords.length < wordCount && currentNode) {
              if (currentNode.nodeType === Node.TEXT_NODE) {
                const text = currentNode.nodeValue.slice(offset).split(/\s+/)
                followingWords = followingWords.concat(text).slice(0, wordCount)
              } else if (
                currentNode.nodeType === Node.ELEMENT_NODE &&
                currentNode.childNodes.length > 0
              ) {
                currentNode = currentNode.childNodes[0]
                offset = 0
                continue
              }
              offset =
                currentNode.nodeType === Node.TEXT_NODE
                  ? currentNode.nodeValue.length
                  : null
              currentNode = currentNode.nextSibling
            }

            return followingWords
          }

          function getSurroundingText(selection, wordCount) {
            const precedingWords = getPrecedingWords(selection, wordCount)
            const followingWords = getFollowingWords(selection, wordCount)
            const encodedPrecedingWords = precedingWords
              .map((word) => encodeURIComponent(word))
              .join(" ")
              .trim()
            const encodedFollowingWords = followingWords
              .map((word) => encodeURIComponent(word))
              .join(" ")
              .trim()
            return `${encodedPrecedingWords}-,${selection.toString()},-${encodedFollowingWords}`
              .trim()
              .replace(/^-,/g, "")
              .replace(/,-$/g, "")
          }

          const selection = window.getSelection()
          if (selection?.rangeCount > 0) {
            const surroundingText = getSurroundingText(selection, 4)
            const url = new URL(window.location.href)
            url.hash = `:~:text=${surroundingText}`
            return {
              text: selection.toString().trim(),
              highlightUrl: url.href
            }
          }
          return { text: "", highlightUrl: "" }
        }
      },
      async (results) => {
        const ret = results[0]?.result
        if (!ret) return
        const translation = await translate(ret.text)
        await createWord({
          word: ret.text,
          url: ret.highlightUrl,
          definition: translation?.text
        })
      }
    )
  }
})

chrome.runtime.onMessage.addListener((request) => {
  if (request.action === "open_url") {
    chrome.tabs.create({ url: request.url })
  } else if (request.type === "logout") {
    store.dispatch(setLogout())
    store.dispatch(resetWords())
  } else if (request.type === "a_few_words_oidc") {
    console.debug("Receive authorization from the web page.", request)
    store.dispatch(setTokens(request))
  }
})
