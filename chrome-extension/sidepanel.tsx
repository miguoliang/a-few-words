import { Provider } from "react-redux"

import { PersistGate } from "@plasmohq/redux-persist/integration/react"

import { launchWebAuthFlow } from "~content"
import { persistor, store, useAppSelector } from "~store"

import "~style.css"

const SidePanel = () => {
  return (
    <Provider store={store}>
      <PersistGate loading={null} persistor={persistor}>
        <View />
      </PersistGate>
    </Provider>
  )
}

const View = () => {
  const accessToken = useAppSelector((state) => state.access_token)
  return (
    <>
      <button
        className="m-5 bg-black text-white"
        onClick={() => launchWebAuthFlow()}>
        Click to authenticate
      </button>
      {accessToken && <p>Access token: {accessToken}</p>}
      {!accessToken && <p>Not authenticated</p>}
    </>
  )
}

export default SidePanel
