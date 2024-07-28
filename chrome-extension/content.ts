import { setTokens } from "~auth-slice"
import { persistor, store } from "~store"

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
  console.log(url)
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
          console.log("Tokens:", tokens)
          store?.dispatch(setTokens(tokens))
        })
        .catch((error) =>
          console.error("Error exchanging code for tokens:", error)
        )
    }
  )
}

export type Word = {
  word: string
  url?: string
  username?: string
}

export type Words = Word[]
