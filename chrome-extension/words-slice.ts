import { createSlice } from "@reduxjs/toolkit"

import { type Words } from "~content"

export interface WordsState {
  words?: Words
}

const wordsSlice = createSlice({
  name: "words",
  initialState: {} as WordsState,
  reducers: {
    setWords: (state, action) => {
      state.words = action.payload
    },
    removeWord: (state, action) => {
      state.words = state.words?.filter((word) => word.id !== action.payload)
    }
  }
})

export const { setWords, removeWord } = wordsSlice.actions

export default wordsSlice.reducer
