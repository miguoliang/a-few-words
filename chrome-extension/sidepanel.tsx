import { AiOutlineLoading } from "react-icons/ai"
import { FaCheck } from "react-icons/fa"
import { HiOutlineTrash } from "react-icons/hi"
import { IoCopyOutline, IoEarth } from "react-icons/io5"
import { Provider } from "react-redux"

import { PersistGate } from "@plasmohq/redux-persist/integration/react"

import { deleteWord, fetchWords, launchWebAuthFlow } from "~content"
import { persistor, store, useAppDispatch, useAppSelector } from "~store"

import "~style.css"

import {
  QueryClient,
  QueryClientProvider,
  useInfiniteQuery,
  useMutation
} from "@tanstack/react-query"
import { ReactQueryDevtools } from "@tanstack/react-query-devtools"
import { motion } from "framer-motion"
import { useEffect, useState } from "react"

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
  const dispatch = useAppDispatch()
  const {
    data,
    fetchNextPage,
    hasNextPage,
    isFetching,
    isFetchingNextPage,
    status
  } = useInfiniteQuery({
    queryKey: ["words"],
    queryFn: ({ pageParam = 0 }) => fetchWords(pageParam),
    initialPageParam: 0,
    getNextPageParam: (lastPage, allPages) => {
      // Determine the next page parameter here
      if (lastPage.length === 10) {
        return allPages.length * 10 // offset for the next page
      } else {
        return undefined // no more pages
      }
    },
    enabled: !!accessToken
  })

  useEffect(() => {
    if (status === "success") {
      dispatch(setWords(data.pages.flatMap((v) => v)))
    }
  }, [status])

  if (!accessToken) {
    return (
      <div className="grid grid-cols-2 gap-1">
        <button
          className="m-5 bg-black text-white rounded p-2"
          onClick={() => launchWebAuthFlow()}>
          Login
        </button>
        <button
          className="m-5 bg-black text-white rounded p-2"
          onClick={() => chrome.runtime.sendMessage({ action: "register" })}>
          Register
        </button>
      </div>
    )
  }

  return (
    <div className="flex flex-col gap-2 p-2">
      <WordList />
      {isFetching && <div>Loading...</div>}
      {isFetchingNextPage && <div>Loading next page...</div>}
      {hasNextPage && (
        <button onClick={() => fetchNextPage()}>Load more</button>
      )}
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

export default SidePanel
