import { createSlice } from "@reduxjs/toolkit"

export interface AuthState {
  access_token?: string
  id_token?: string
  refresh_token?: string
  expires_in?: number
  username?: string
}

const authSlice = createSlice({
  name: "auth",
  initialState: {} as AuthState,
  reducers: {
    setTokens: (state, action) => {
      state.access_token = action.payload.access_token
      state.id_token = action.payload.id_token
      state.refresh_token = action.payload.refresh_token
      state.expires_in = action.payload.expires_in
    }
  }
})

export const { setTokens } = authSlice.actions

export default authSlice.reducer
