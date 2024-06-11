import { CardPreview } from '@components/card-preview'
import { Button } from '@components/ui/button'
import { Label } from '@components/ui/label'
import { Textarea } from '@components/ui/textarea'
import { invoke } from "@tauri-apps/api/core"
import { ArrowLeft } from 'phosphor-react'
import { useEffect, useState } from 'react'
import { useNavigate, useParams } from 'react-router-dom'

interface Card {
  id: number
  deck_id: string
  front: string
  back: string
}

export const Card = () => {
  const [card, setCard] = useState<Card>({} as Card)
  const [isLoading, setIsLoading] = useState(true)

  const { cardId } = useParams()
  const navigate = useNavigate()

  function handleUpdateFlashcard() {
    try {
      invoke('update_flashcard', { id: card.id, front: card.front, back: card.back }).then(() => {
        return navigate(`/decks/${card.deck_id}`)
      })

    } catch (err) {
      console.log(err)
    }
  }

  useEffect(() => {
    invoke<Card>('find_flashcard_by_id', { id: cardId }).then((data) => {
      setCard(data)
      setIsLoading(false)
    })
  }, [cardId])

  // If we don't make this fallback CardPreview tries to render and execute
  // hook useFurigana with empty data, throwing error
  // TODO fix 1: make possible to useFurigana receive empty data and
  // don't throw any error (lol)
  // TODO fix 2: make use of loading states like now but implement some
  // loading skeleton to improve UX
  if (isLoading) {
    return <p>Loading...</p>
  }

  return (
    <div className="p-6">
      <header className="flex justify-between">
        <Button variant="outline" size="icon" onClick={() => navigate('/')}>
          <ArrowLeft className="h-6 w-6" />
        </Button>
      </header>

      <h1 className="my-6 text-3xl tracking-tighter">Editar flashcard</h1>

      {/* TODO: Add toolbar */}

      <div className="space-y-2">
        <div>
          <Label htmlFor="front-card">Front</Label>
          <Textarea
            className="mt-1 resize-none"
            placeholder="A frente do seu card."
            id="front-card"
            onChange={(event) =>
              setCard((prevState) => {
                return {
                  ...prevState,
                  front: event.target.value,
                }
              })
            }
          >{card.front}</Textarea>
        </div>

        <div>
          <Label htmlFor="back-card">Back</Label>
          <Textarea
            className="mt-1 resize-none"
            placeholder="Atrás do seu card."
            id="back-card"
            onChange={(event) =>
              setCard((prevState) => {
                return {
                  ...prevState,
                  back: event.target.value,
                }
              })
            }
          >{card.back}</Textarea>
        </div>
      </div>

      <p className="mb-2 mt-6 text-lg font-medium">Preview</p>
      <div>
        <CardPreview data={card} />
      </div>

      <Button className="mt-6 w-full" onClick={handleUpdateFlashcard}>
        Salvar
      </Button>
    </div>
  )
}
