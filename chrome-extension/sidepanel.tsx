import { Provider } from "react-redux"

import { PersistGate } from "@plasmohq/redux-persist/integration/react"

import { fetchWords } from "~common"
import { persistor, store, useAppSelector } from "~store"

import "~style.css"

import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { ReactQueryDevtools } from "@tanstack/react-query-devtools"
import { HashRouter, Navigate, Outlet, Route, Routes } from "react-router-dom"

import { setLogout, setTokens } from "~auth-slice"
import HeaderView from "~HeaderView"
import WelcomeView from "~WelcomeView"
import WordListView from "~WordListView"
import { resetWords, setWords } from "~words-slice"

const queryClient = new QueryClient()

const SidePanel = () => {
  return (
    <QueryClientProvider client={queryClient}>
      <Provider store={store}>
        <PersistGate loading={null} persistor={persistor}>
          <HashRouter>
            <Routes>
              <Route element={<Layout />}>
                <Route path="/word-list" element={<WordListView />} />
              </Route>
              <Route path="/" element={<Navigate to="/welcome" />} />
              <Route path="/welcome" element={<WelcomeView />} />
            </Routes>
          </HashRouter>
        </PersistGate>
      </Provider>
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  )
}

const Layout = () => {
  const accessToken = useAppSelector((state) => state.auth.access_token)
  const idToken = useAppSelector((state) => state.auth.id_token)
  if (!accessToken) {
    return <Navigate to="/welcome" />
  }
  return (
    <div className="flex flex-col gap-2 p-2">
      <HeaderView idToken={idToken} />
      <Outlet />
    </div>
  )
}

// Listen for messages from the background script
chrome.runtime.onMessage.addListener(async (message) => {
  if (message.type === "wordCreated") {
    const words = store.getState().words.words ?? []
    const latestWords = await fetchWords(message.word)
    if (words.length == 0) {
      store.dispatch(setWords([...latestWords]))
      return
    }
    const firstWord = words.at(0)
    const newWords = latestWords.filter((word) => word.id > firstWord.id)
    store.dispatch(setWords([...newWords, ...words]))
  } else if (message.type === "logout") {
    store.dispatch(setLogout())
    store.dispatch(resetWords())
  } else if (message.type === "a_few_words_oidc") {
    store.dispatch(setTokens(message))
  }
})

export default SidePanel
