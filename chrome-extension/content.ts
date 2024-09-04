import { type Message } from "~common"

export {}

window.addEventListener("message", (event) => {
  if (event.source !== window) {
    return
  }
  const message = event.data as Message
  if (message.type === "a_few_words_oidc") {
    console.debug("Received tokens from oidc provider", message)
    chrome.runtime.sendMessage(message)
  }
})

chrome.runtime.onMessage.addListener((request: Message) => {
  console.debug("Received message from the background script", request)
  if (request.type === "logout") {
    window.postMessage(request, "*")
  }
})
