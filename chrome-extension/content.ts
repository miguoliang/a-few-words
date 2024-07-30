import { setLogout, setTokens } from "~auth-slice"
import { store } from "~store"
import { setWords } from "~words-slice"

export {}

const HOST =
  "https://broccoli-go-user-pool-domain.auth.us-east-1.amazoncognito.com"

export function launchWebAuthFlow() {
  const url =
    `${HOST}/oauth2/authorize?` +
    "client_id=5p99s5nl7nha5tfnpik3r0rb7j&" +
    "response_type=code&" +
    "redirect_uri=" +
    encodeURIComponent(chrome.identity.getRedirectURL()) +
    "&" +
    `scope=${encodeURIComponent("openid profile email")}`
  chrome.identity.launchWebAuthFlow(
    {
      url,
      interactive: true
    },
    (redirectUrl) => {
      if (chrome.runtime.lastError || redirectUrl.includes("error")) {
        console.error(chrome.runtime.lastError)
        console.error("Error during authentication:", redirectUrl)
        return
      }

      // Extract authorization code from redirectUrl
      let url = new URL(redirectUrl)
      let code = url.searchParams.get("code")

      // Exchange authorization code for tokens
      fetch(`${HOST}/oauth2/token`, {
        method: "POST",
        headers: {
          "Content-Type": "application/x-www-form-urlencoded"
        },
        body: new URLSearchParams({
          code: code,
          client_id: "5p99s5nl7nha5tfnpik3r0rb7j",
          redirect_uri: chrome.identity.getRedirectURL(),
          grant_type: "authorization_code"
        })
      })
        .then((response) => response.json())
        .then((tokens) => {
          // Handle tokens (access_token, id_token, etc.)
          store?.dispatch(setTokens(tokens))
        })
        .catch((error) =>
          console.error("Error exchanging code for tokens:", error)
        )
    }
  )
}

export const fetchWords = async () => {
  return fetch("http://localhost:8000/words", {
    headers: {
      Authorization: `Bearer ${store.getState().auth.access_token}`
    }
  }).then(async (response) => {
    if (response.ok) {
      const data = await response.json()
      store.dispatch(setWords(data))
    } else if (response.status === 401) {
      store.dispatch(setLogout())
    } else {
      console.error(`Error: ${response.statusText} (${response.status})`)
    }
  })
}

export const createWord = async (word: Word) => {
  return fetch("http://localhost:8000/words", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + store?.getState().auth.access_token
    },
    body: JSON.stringify(word)
  }).then(async (response) => {
    if (response.ok) {
      const words = store.getState().words.words
      store.dispatch(setWords([word, ...words]))
    } else if (response.status === 401) {
      store.dispatch(setLogout())
    } else {
      console.error(`Error: ${response.statusText} (${response.status})`)
    }
  })
}

export type Word = {
  word: string
  url?: string
  username?: string
}

export type Words = Word[]
