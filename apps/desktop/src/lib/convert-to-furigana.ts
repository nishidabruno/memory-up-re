// input: 初[はじ]めまして、 佐[さ]藤[とう]です
// output: <ruby>初<rt>はじ</rt>めまして</ruby>、<ruby>佐藤<rt>さとう</rt></ruby>です

/**
 * Converts and returns a string to furigana using the HTML `ruby` tag.
 *
 * @param text - The text input using `[ ]` syntax.
 * @example
 * ```ts
 * const furigana = useFurigana('初[はじ]めまして')
 * ```
 * @returns
 * ```html
 * <ruby>初<rt>はじ</rt>めまして</ruby>
 * ```
 */
export const convertToFurigana = (text: string) => {
  if (!text) return ''
  let startFromIndex = 0
  let currentText = text

  while (startFromIndex !== -1) {
    const { newString, closeBracketIndex } = convertToFuriganaHtml(
      currentText,
      startFromIndex,
    )

    currentText = newString
    startFromIndex = closeBracketIndex
  }

  return currentText
}

const convertToFuriganaHtml = (text: string, startFrom: number) => {
  const openBracketIndex = text.indexOf('[', startFrom)
  if (openBracketIndex === -1) {
    return { newString: text, closeBracketIndex: -1 }
  }
  const closeBracketIndex = text.indexOf(']', openBracketIndex - 1)

  const convertedString = `<ruby>${text.slice(
    openBracketIndex - 1,
    openBracketIndex,
  )}<rt>${text.slice(
    openBracketIndex + 1,
    closeBracketIndex,
  )}</rt></ruby>${text.slice(closeBracketIndex + 1)}`
  let newString = ''
  if (openBracketIndex > 2) {
    newString = `${text.slice(0, openBracketIndex - 2)}${convertedString}`
    return { newString, closeBracketIndex }
  }
  newString = convertedString

  return { newString, closeBracketIndex }
}
