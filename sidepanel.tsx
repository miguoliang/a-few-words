import { useStorage } from "@plasmohq/storage/hook"

const SidePanel = () => {
  const [text] = useStorage("text");
  return (
    <div className="side-panel">
      <h2>{text}</h2>
    </div>
  )
}

export default SidePanel
