import { PiSignOutBold } from "react-icons/pi"

import { WEBSITE_LOGIN_URL, type Message } from "~common"

interface HeaderViewProps {
  idToken: string
}

const HeaderView = ({ idToken }: Readonly<HeaderViewProps>) => {
  const jwt = idToken.split(".")[1]
  const decoded = JSON.parse(atob(jwt))
  return (
    <div className="flex justify-between items-center p-2">
      <span className="text-lg">
        <span className="font-bold">Hey!</span> 👋 {decoded.name}
      </span>
      <button
        className="text-gray-500 text-lg"
        onClick={() => sendLogoutMessage()}>
        <PiSignOutBold />
      </button>
    </div>
  )
}

function sendLogoutMessage() {
  chrome.runtime.sendMessage({ type: "logout" } as Message)
  chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
    for (const tab of tabs) {
      if (tab?.id) {
        chrome.tabs.sendMessage(tabs[0].id, { type: "logout" } as Message)
      }
    }
  })
}

export default HeaderView
