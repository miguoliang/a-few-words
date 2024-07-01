import { useStorage } from "@plasmohq/storage/hook"

import type { CardList } from "~content"

const SidePanel = () => {
  const [cards] = useStorage<CardList>("cards", [])
  return (
    <div>
      {cards.map((card, index) => (
        <div key={index}>
          <div>{card.front}</div>
          <div>{card.back}</div>
        </div>
      ))}
    </div>
  )
}

export default SidePanel
