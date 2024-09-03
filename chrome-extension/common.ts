import { setLogout } from "~auth-slice"
import { store } from "~store"
import { setHasMore, setIsLoading, setWords } from "~words-slice"

export {}

const PAGE_SIZE = 50

export type MessageName =
  | "word_created"
  | "logout"
  | "a_few_words_oidc"
  | "open_url"

export interface Message {
  type: MessageName
  [key: string]: any
}

export const COOKIES_DOMAIN =
  process.env.PLASMO_PUBLIC_COOKIES_DOMAIN ||
  (() => {
    throw new Error("COOKIES_DOMAIN is required")
  })()

export const WEBSITE_LOGIN_URL =
  process.env.PLASMO_PUBLIC_WEBSITE_LOGIN_URL ||
  (() => {
    throw new Error("WEBSITE_LOGIN_URL is required")
  })()

export const OIDC_AUTHORITY =
  process.env.PLASMO_PUBLIC_OIDC_AUTHORITY ||
  (() => {
    throw new Error("OIDC_AUTHORITY is required")
  })()

export const OIDC_HOST =
  process.env.PLASMO_PUBLIC_OIDC_HOST ||
  (() => {
    throw new Error("OIDC_HOST is required")
  })()

export const OIDC_CLIENT_ID =
  process.env.PLASMO_PUBLIC_OIDC_CLIENT_ID ||
  (() => {
    throw new Error("OIDC_CLIENT_ID is required")
  })()

export const OIDC_REDIRECT_URI =
  process.env.PLASMO_PUBLIC_OIDC_REDIRECT_URI ||
  (() => {
    throw new Error("OIDC_REDIRECT_URI is required")
  })()

export const OIDC_POST_LOGOUT_REDIRECT_URI =
  process.env.PLASMO_PUBLIC_OIDC_POST_LOGOUT_REDIRECT_URI ||
  (() => {
    throw new Error("OIDC_POST_LOGOUT_REDIRECT_URI is required")
  })()

export const API_HOST =
  process.env.PLASMO_PUBLIC_API_HOST ||
  (() => {
    throw new Error("API_HOST is required")
  })()

export const loadMoreWords = async () => {
  const isLoading = store.getState().words.isLoading
  if (isLoading) return
  const words = store.getState().words?.words ?? []
  const offset = words?.length ?? 0
  store.dispatch(setIsLoading(true))
  try {
    const newWords = await fetchWords(offset)
    store.dispatch(setWords([...words, ...newWords]))
    store.dispatch(setHasMore(newWords.length >= PAGE_SIZE))
  } finally {
    store.dispatch(setIsLoading(false))
  }
}

export const fetchWords = async (
  offset: number = 0,
  size: number = PAGE_SIZE
) => {
  try {
    const response = await fetch(
      `${API_HOST}/api/v1/words?offset=${offset}&size=${size}`,
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
    const response = await fetch(`${API_HOST}/api/v1/words`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer " + store?.getState().auth.access_token
      },
      body: JSON.stringify(word)
    })
    if (response.ok) {
      await response.json()
      chrome.runtime.sendMessage({ type: "word_created" })
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
    const response = await fetch(`${API_HOST}/api/v1/words/${id}`, {
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
    const response = await fetch(`${API_HOST}/api/v1/translate?text=${text}`, {
      headers: {
        Authorization: `Bearer ${store.getState().auth.access_token}`
      }
    })
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

export function isExpired(jwt: string) {
  try {
    const claims = jwt.split(".")[1]
    const decoded = JSON.parse(atob(claims))
    const currentTime = Math.floor(Date.now() / 1000)
    return decoded.exp < currentTime
  } catch {
    return true
  }
}
