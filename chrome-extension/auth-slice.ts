import { createSlice } from "@reduxjs/toolkit"

export interface AuthState {
  access_token?: string
  id_token?: string
  refresh_token?: string
  expires_in?: number
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
    },
    setLogout: (state) => {
      state.access_token = undefined
      state.id_token = undefined
      state.refresh_token = undefined
      state.expires_in = undefined
    }
  }
})

export const { setTokens, setLogout } = authSlice.actions

export default authSlice.reducer
