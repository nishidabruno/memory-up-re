import { Button } from '@components/ui/button'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@components/ui/table'
import { convertToFurigana } from '@lib/convert-to-furigana'
import { invoke } from "@tauri-apps/api/core"
import { ArrowLeft, Plus } from 'phosphor-react'
import { useEffect, useState } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import { toast } from 'sonner'

interface Deck {
  id: string
  title: string
}

interface Card {
  id: number
  front: string
  back: string
}

export const Deck = () => {
  const { deckId } = useParams()
  const navigate = useNavigate()
  const [deck, setDeck] = useState<Deck>()
  const [cards, setCards] = useState<Card[]>()

  useEffect(() => {
    async function loadData() {
      try {
        const deck = await invoke<Deck>('find_deck_by_id', { id: deckId })
        setDeck(deck)
      } catch (err) {
        toast.error(`Erro ao carregar deck. Tente novamente mais tarde. ${err}`)
      }

      try {
        const flashcards = await invoke<Card[]>('find_flashcards_by_deck_id', {
          deckId,
        })
        setCards(flashcards)
      } catch (err) {
        toast.error(
          `Erro ao carregar flashcards. Tente novamente mais tarde. ${err}`,
        )
      }
    }

    loadData()
  }, [deckId])

  return (
    <div className="p-6">
      <header className="flex justify-between">
        <Button variant="outline" size="icon" onClick={() => navigate("/")}>
          <ArrowLeft className="h-6 w-6" />
        </Button>
        <Button onClick={() => navigate(`/createCard/${deckId}`)} size="sm">
          <Plus className="mr-2 h-4 w-4" />
          Novo card
        </Button>
      </header>

      <h1 className="my-6 text-3xl tracking-tighter">{deck?.title}</h1>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Frente</TableHead>
            <TableHead>Verso</TableHead>
          </TableRow>
        </TableHeader>

        <TableBody>
          {cards?.map((card) => (
            <TableRow
              key={card.id}
              onClick={() => navigate(`cards/${card.id}`)}
              className="cursor-pointer text-base"
            >
              <TableCell className="max-w-[313px] overflow-hidden text-ellipsis whitespace-nowrap">
                {card.front}
              </TableCell>
              <TableCell
                className="max-w-[313px] overflow-hidden text-ellipsis whitespace-nowrap"
                dangerouslySetInnerHTML={{
                  __html: convertToFurigana(card.back),
                }}
              ></TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  )
}
