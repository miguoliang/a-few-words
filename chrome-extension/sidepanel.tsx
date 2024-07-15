import { useStorage } from "@plasmohq/storage/hook"

import "~style.css"

import type { SentenceList } from "~content"

const SidePanel = () => {
  const [cards, setCards] = useStorage<SentenceList>("cards", [])
  const removeCard = (index: number) => {
    const newCards = [...cards]
    newCards.splice(index, 1)
    setCards(newCards)
  }
  return (
    <div className="p-2 gap-2 flex flex-col">
      {cards.map((card, index) => (
        <details className="collapse bg-base-200" key={index}>
          <summary className="collapse-title text-md font-medium bg-green-200">
            {card.front}
          </summary>
          <div className="collapse-content">
            <p>content</p>
          </div>
        </details>
      ))}
    </div>
  )
}

export default SidePanel
