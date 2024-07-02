import cssText from "data-text:~style.css"

export const getStyle = () => {
  const style = document.createElement("style")
  style.textContent = cssText.replaceAll(':root', ':host(plasmo-csui)');
  return style
}

export type Sentence = {
  front: string
  back: string
}

export type SentenceList = Sentence[]
