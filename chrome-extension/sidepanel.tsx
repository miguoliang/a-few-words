import banner from "data-base64:~assets/banner.webp"
import { AiOutlineLoading } from "react-icons/ai"
import { FaCheck } from "react-icons/fa"
import { HiOutlineTrash } from "react-icons/hi"
import { IoCopyOutline, IoEarth } from "react-icons/io5"
import { PiSignOutBold } from "react-icons/pi"
import { Provider } from "react-redux"

import { PersistGate } from "@plasmohq/redux-persist/integration/react"

import { deleteWord, launchWebAuthFlow, loadMoreWords } from "~content"
import { persistor, store, useAppDispatch, useAppSelector } from "~store"

import "~style.css"

import {
  QueryClient,
  QueryClientProvider,
  useMutation
} from "@tanstack/react-query"
import { ReactQueryDevtools } from "@tanstack/react-query-devtools"
import { motion } from "framer-motion"
import { useEffect, useState } from "react"
import { useInView } from "react-intersection-observer"

import { removeWord, setWords } from "~words-slice"

const queryClient = new QueryClient()

const SidePanel = () => {
  return (
    <QueryClientProvider client={queryClient}>
      <Provider store={store}>
        <PersistGate loading={null} persistor={persistor}>
          <AuthenticatedView />
        </PersistGate>
      </Provider>
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  )
}

const AuthenticatedView = () => {
  const accessToken = useAppSelector((state) => state.auth.access_token)
  const isLoading = useAppSelector((state) => state.words.isLoading)
  const hasMore = useAppSelector((state) => state.words.hasMore)
  const { ref, inView } = useInView()

  useEffect(() => {
    if (inView) {
      loadMoreWords().then(() => {})
    }
  }, [inView])

  if (!accessToken) {
    return (
      <div className="flex flex-col items-stretch justify-center gap-8 px-8 -mt-16 h-[100vh]">
        <div className="overflow-hidden h-[200px] rounded-3xl border-[5px] border-black border-solid">
          <img src={banner} alt="logo" className="mt-[-30px]" />
        </div>
        <button
          className="m-5 bg-black block text-white rounded-full text-xl p-3"
          onClick={() => launchWebAuthFlow()}>
          💪<span className="ml-5">Get Started</span>
        </button>
      </div>
    )
  }

  return (
    <div className="flex flex-col gap-2 p-2">
      <Header accessToken={accessToken} />
      <WordList />
      <button
        ref={ref}
        onClick={async () => await loadMoreWords()}
        disabled={!hasMore || isLoading}>
        {isLoading
          ? "Loading more..."
          : hasMore
            ? "Load Newer"
            : "Nothing more to load"}
      </button>
    </div>
  )
}

const WordList = () => {
  const words = useAppSelector((state) => state.words.words)
  return (
    <div className="flex flex-col gap-2">
      {words?.length === 0 && <div>No words saved yet</div>}
      {words?.map((word) => (
        <WordCell
          key={word.id}
          word={word.word}
          id={word.id}
          url={word.url}
          definition={word.definition}
        />
      ))}
    </div>
  )
}

interface WordProps {
  id: number
  word: string
  url?: string
  definition?: string
}

const WordCell = ({ id, word, url, definition }: WordProps) => {
  return (
    <div className="flex flex-col bg-gray-200 p-2 rounded-lg gap-1">
      <WordToolbar id={id} word={word} url={url} />
      <span>{word}</span>
      {definition && <span>{definition}</span>}
    </div>
  )
}

const WordToolbar = ({ id, word, url }: WordProps) => {
  const dispatch = useAppDispatch()
  const mutation = useMutation({
    mutationFn: (id: number) => deleteWord(id),
    onSuccess: () => {
      dispatch(removeWord(id))
    }
  })
  return (
    <div className="flex justify-end gap-2">
      <button
        type="button"
        onClick={async () => await mutation.mutateAsync(id)}>
        {mutation.isIdle && <HiOutlineTrash />}
        {mutation.isPending && (
          <motion.div
            animate={{ rotate: 360 }}
            transition={{
              duration: 1,
              bounce: 0,
              repeat: Infinity,
              type: "spring",
              delay: 0
            }}>
            <AiOutlineLoading />
          </motion.div>
        )}
      </button>
      <button
        type="button"
        disabled={!url}
        onClick={() =>
          chrome.runtime.sendMessage({
            action: "openUrl",
            url
          })
        }>
        <IoEarth />
      </button>
      <CopyButton text={word} />
    </div>
  )
}

const CopyButton = ({ text }: { text: string }) => {
  const [copied, setCopied] = useState(false)
  return (
    <button
      type="button"
      onClick={() => {
        navigator.clipboard.writeText(text)
        setCopied(true)
        setTimeout(() => setCopied(false), 1000)
      }}>
      {copied ? <FaCheck /> : <IoCopyOutline />}
    </button>
  )
}

interface HeaderProps {
  accessToken: string
}

const Header = ({ accessToken }: HeaderProps) => {
  const jwt = accessToken.split(".")[1]
  const decoded = JSON.parse(atob(jwt))
  return (
    <div className="flex justify-between items-center p-2">
      <span className="text-lg">
        <span className="font-bold">Hey!</span> 👋 {decoded.username}
      </span>
      <button
        className="text-gray-500 text-lg"
        onClick={() => chrome.runtime.sendMessage({ action: "logout" })}>
        <PiSignOutBold />
      </button>
    </div>
  )
}

// Listen for messages from the background script
chrome.runtime.onMessage.addListener((message) => {
  if (message.type === "wordCreated") {
    loadMoreWords().then(() => {})
  }
})

export default SidePanel
