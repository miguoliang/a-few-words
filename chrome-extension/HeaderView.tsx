import { PiSignOutBold } from "react-icons/pi"

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
        onClick={() => chrome.runtime.sendMessage({ action: "logout" })}>
        <PiSignOutBold />
      </button>
    </div>
  )
}

export default HeaderView