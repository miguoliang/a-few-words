import { createSlice } from "@reduxjs/toolkit"

import type { Words } from "~content"

export interface WordsState {
  words?: Words
}

const wordsSlice = createSlice({
  name: "words",
  initialState: {} as WordsState,
  reducers: {
    setWords: (state, action) => {
      state.words = action.payload
    }
  }
})

export const { setWords } = wordsSlice.actions

export default wordsSlice.reducer
