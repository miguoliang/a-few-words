import { createSlice } from "@reduxjs/toolkit"

export interface AuthState {
  access_token?: string
  id_token?: string
  refresh_token?: string
}

const authSlice = createSlice({
  name: "auth",
  initialState: {} as AuthState,
  reducers: {
    setTokens: (state, action) => {
      console.log('setTokens', action.payload)
      state.access_token = action.payload.access_token
      state.id_token = action.payload.id_token
      state.refresh_token = action.payload.refresh_token
    }
  }
})

export const { setTokens } = authSlice.actions

export default authSlice.reducer
