import { setLogout, setTokens } from "~auth-slice"
import { store } from "~store"
import { setHasMore, setIsLoading, setWords } from "~words-slice"

export {}

export const AUTH_HOST =
  "https://broccoli-go-user-pool-domain.auth.us-east-1.amazoncognito.com"

export const AUTH_CLIENT_ID = "5p99s5nl7nha5tfnpik3r0rb7j"

export function launchWebAuthFlow() {
  const searchParams = new URLSearchParams()
  searchParams.append("client_id", AUTH_CLIENT_ID)
  searchParams.append("response_type", "code")
  searchParams.append("redirect_uri", chrome.identity.getRedirectURL())
  searchParams.append("scope", "openid email profile")
  const url = `${AUTH_HOST}/oauth2/authorize?${searchParams.toString()}`
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
      fetch(`${AUTH_HOST}/oauth2/token`, {
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

export const loadMoreWords = async () => {
  const isLoading = store.getState().words.isLoading
  if (isLoading) return
  const words = store.getState().words?.words ?? []
  const offset = words?.length ?? 0
  console.log("Loading more words...", offset, words)
  store.dispatch(setIsLoading(true))
  try {
    const newWords = await fetchWords(offset)
    store.dispatch(setWords([...words, ...newWords]))
    store.dispatch(setHasMore(newWords.length >= 10))
  } finally {
    store.dispatch(setIsLoading(false))
  }
}

const fetchWords = async (offset: number = 0, size: number = 10) => {
  try {
    const response = await fetch(
      `http://localhost:8000/api/v1/words?offset=${offset}&size=${size}`,
      {
        headers: {
          Authorization: `Bearer ${store.getState().auth.access_token}`
        }
      }
    )
    if (response.ok) {
      return (await response.json()) as Words
    } else if (response.status === 401) {
      store.dispatch(setLogout())
      return [] as Words
    } else {
      throw Error(`Error: ${response.statusText} (${response.status})`)
    }
  } catch (error) {
    console.error("Error fetching words:", error)
    return [] as Words
  }
}

export const createWord = async (word: Word) => {
  try {
    const response = await fetch("http://localhost:8000/api/v1/words", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + store?.getState().auth.access_token
      },
      body: JSON.stringify(word)
    })
    if (response.ok) {
      await response.json()
      chrome.runtime.sendMessage({ type: "wordCreated" })
    } else if (response.status === 401) {
      store.dispatch(setLogout())
    } else {
      console.error("Error creating word:", response.statusText)
    }
  } catch (error) {
    console.error("Error creating word:", error)
  }
}

export const deleteWord = async (id: number) => {
  try {
    const response = await fetch(`http://localhost:8000/api/v1/words/${id}`, {
      method: "DELETE",
      headers: {
        Authorization: "Bearer " + store?.getState().auth.access_token
      }
    })
    if (response.ok) {
      store.dispatch(
        setWords(store.getState().words.words.filter((word) => word.id !== id))
      )
    } else if (response.status === 401) {
      store.dispatch(setLogout())
    } else {
      console.error("Error deleting word:", response.statusText)
    }
  } catch (error) {
    console.error("Error deleting word:", error)
  }
}

export type TranslateResponse = {
  text: string
}

export const translate = async (text: string) => {
  try {
    const response = await fetch(
      `http://localhost:8000/api/v1/translate?text=${text}`,
      {
        headers: {
          Authorization: `Bearer ${store.getState().auth.access_token}`
        }
      }
    )
    if (response.ok) {
      return (await response.json()) as TranslateResponse
    } else if (response.status === 401) {
      store.dispatch(setLogout())
    } else {
      console.error("Error translating text:", response.statusText)
    }
  } catch (error) {
    console.error("Error translating text:", error)
  }
}

export type Word = {
  id?: number
  word: string
  definition?: string
  url?: string
  username?: string
  created_at?: number
  updated_at?: number
}

export type Words = Word[]
