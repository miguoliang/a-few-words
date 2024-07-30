import { Provider } from "react-redux"

import { PersistGate } from "@plasmohq/redux-persist/integration/react"

import { launchWebAuthFlow } from "~content"
import { persistor, store, useAppDispatch, useAppSelector } from "~store"
import { fetchWords } from "~content"

import "~style.css"

import { useEffect } from "react"

const SidePanel = () => {
  return (
    <Provider store={store}>
      <PersistGate loading={null} persistor={persistor}>
        <AuthenticatedView />
      </PersistGate>
    </Provider>
  )
}

const AuthenticatedView = () => {
  const accessToken = useAppSelector((state) => state.auth.access_token)
  const dispatch = useAppDispatch()

  useEffect(() => {
    if (!accessToken) return
    fetchWords()
  }, [accessToken, dispatch])

  return (
    <div className="flex flex-col p-2">
      {!accessToken && (
        <button
          className="m-5 bg-black text-white"
          onClick={() => launchWebAuthFlow()}>
          Click to authenticate
        </button>
      )}
      <WordList />
    </div>
  )
}

const WordList = () => {
  const words = useAppSelector((state) => state.words.words)
  console.log(words)
  return (
    <div className="flex flex-col">
      {words?.length === 0 && <div>No words saved yet</div>}
      {words?.map((word, index) => <WordCell key={index} word={word.word} />)}
    </div>
  )
}

interface WordProps {
  word: string
}

const WordCell = ({ word }: WordProps) => {
  return <div>{word}</div>
}

export default SidePanel
