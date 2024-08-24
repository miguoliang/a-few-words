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
    hasMore: true
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
    }
  }
})

export const { setWords, removeWord, setIsLoading, setHasMore } =
  wordsSlice.actions

export default wordsSlice.reducer
