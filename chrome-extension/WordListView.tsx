import { useMutation } from "@tanstack/react-query"
import { motion } from "framer-motion"
import { useEffect, useState } from "react"
import { AiOutlineLoading } from "react-icons/ai"
import { FaCheck } from "react-icons/fa"
import { HiOutlineTrash } from "react-icons/hi"
import { IoCopyOutline, IoEarth } from "react-icons/io5"
import { useInView } from "react-intersection-observer"

import { deleteWord, loadMoreWords } from "~content"
import { useAppDispatch, useAppSelector } from "~store"
import { removeWord } from "~words-slice"

const WordListView = () => {
  const words = useAppSelector((state) => state.words.words)
  const isLoading = useAppSelector((state) => state.words.isLoading)
  const hasMore = useAppSelector((state) => state.words.hasMore)
  const { ref, inView } = useInView()

  useEffect(() => {
    if (inView || words?.length === 0) {
      loadMoreWords().then(() => {})
    }
  }, [inView])

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
      {words?.length === 0 && (
        <button
          ref={ref}
          onClick={async () => await loadMoreWords()}
          disabled={!hasMore || isLoading}>
          {isLoading && "Loading more..."}
          {!isLoading && (hasMore ? "Load Newer" : "Nothing more to load")}
        </button>
      )}
    </div>
  )
}

interface WordProps {
  id: number
  word: string
  url?: string
  definition?: string
}

const WordCell = ({ id, word, url, definition }: Readonly<WordProps>) => {
  return (
    <div className="flex flex-col bg-gray-200 p-2 rounded-lg gap-1">
      <WordToolbar id={id} word={word} url={url} />
      <span>{word}</span>
      {definition && <span>{definition}</span>}
    </div>
  )
}

interface WordToolbarProps {
  id: number
  word: string
  url?: string
}

const WordToolbar = ({ id, word, url }: Readonly<WordToolbarProps>) => {
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

export default WordListView
