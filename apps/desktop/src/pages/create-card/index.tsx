import { CardPreview } from '@components/card-preview'
import { Button } from '@components/ui/button'
import { Label } from '@components/ui/label'
import { Textarea } from '@components/ui/textarea'
import { invoke } from "@tauri-apps/api/core"
import { ArrowLeft } from 'phosphor-react'
import { useState } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import { toast } from 'sonner'

interface Card {
  front: string
  back: string
}

export const CreateCard = () => {
  const [card, setCard] = useState<Card>({ front: '', back: '' })

  const { deckId } = useParams()
  const navigate = useNavigate()

  const handleCreateCard = async () => {
    if (card.front === '') {
      toast.warning('A frente do seu card não pode estar vazio.')
      return
    }
    if (card.back === '') {
      toast.warning('Atrás do seu card não pode estar vazio.')
      return
    }

    try {
      await invoke('create_flashcard', {
        front: card.front,
        back: card.back,
        deckId,
      })
    } catch (err) {
      toast.error(`Erro ao criar o card. Tente novamamente mais tarde. ${err}`)
      return
    }

    toast.success('Card criado com sucesso!')
    navigate(-1)
  }

  return (
    <div className="p-6">
      <header className="flex justify-between">
        <Button variant="outline" size="icon" onClick={() => navigate('/')}>
          <ArrowLeft className="h-6 w-6" />
        </Button>
      </header>

      <h1 className="my-6 text-3xl tracking-tighter">Novo card</h1>

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
          />
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
          />
        </div>
      </div>

      <p className="mb-2 mt-6 text-lg font-medium">Preview</p>
      <div>
        <CardPreview data={card} />
      </div>

      <Button className="mt-6 w-full" onClick={handleCreateCard}>
        Criar card
      </Button>
    </div>
  )
}
