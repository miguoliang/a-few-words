import { createSlice } from "@reduxjs/toolkit"

import { type Words } from "~content"

export interface WordsState {
  words: Words
  isLoading: boolean
  hasMore: boolean
}

const wordsSlice = createSlice({
  name: "words",
  initialState: {
    words: [],
    isLoading: false,
    hasMore: false
  } as WordsState,
  reducers: {
    setWords: (state, action) => {
      state.words = action.payload
    },
    removeWord: (state, action) => {
      state.words = state.words?.filter((word) => word.id !== action.payload)
    },
    setIsLoading: (state, action) => {
      state.isLoading = action.payload
    },
    setHasMore: (state, action) => {
      state.hasMore = action.payload
    },
    resetWords: (state) => {
      state.words = []
      state.isLoading = false
      state.hasMore = false
    }
  }
})

export const { setWords, removeWord, setIsLoading, setHasMore, resetWords } =
  wordsSlice.actions

export default wordsSlice.reducer
