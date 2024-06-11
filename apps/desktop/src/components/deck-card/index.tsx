import { DeckDialog } from '@components/deck-card/deck-card-dialog'
import { Dialog, DialogTrigger } from '@components/ui/dialog'
import { invoke } from "@tauri-apps/api/core"
import { useEffect, useState } from "react"

interface DeckCardProps {
  data: {
    title: string
    id: string
  }
}

export const DeckCard = ({ data }: DeckCardProps) => {
  const [correctPercentage, setCorrectPercentage] = useState(0)
  const [totalFlashcards, setTotalFlashcards] = useState(0)

  async function get_correct_percentage() {
    const res = await invoke<number>('get_correct_percentage', { deckId: data.id })
    setCorrectPercentage(res)
  }
  
  async function get_total_flashcards_by_deck() {
    const res = await invoke<number>('get_total_flashcards_by_deck', { deckId: data.id })
    setTotalFlashcards(res)
  }

  useEffect(() => {
    get_correct_percentage()
    get_total_flashcards_by_deck()
  }, [])

  return (
    <Dialog>
      <DialogTrigger asChild>
        <div className="p2 w-52 cursor-pointer rounded-md bg-muted border-2 border-border transition-all hover:brightness-90">
          <div className="flex flex-col items-center gap-1 rounded-md rounded-b-none border bg-gradient-to-tl from-gray-900 to-indigo-900 p-8">
            <p className="font-medium text-sm text-muted-foreground">10 sobrando</p>
            <h2 className="text-xl font-bold text-primary">{data.title}</h2>
          </div>

          <div className="flex flex-col gap-2 p-4">
            <div className="flex justify-between">
              <span className="text-sm text-muted-foreground">
                Total de cards
              </span>
              <span className="text-sm text-primary">{totalFlashcards}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-sm text-muted-foreground">Acertos</span>
              <span className="text-sm text-primary">{correctPercentage}%</span>
            </div>
          </div>
        </div>
      </DialogTrigger>

      <DeckDialog data={data} />
    </Dialog>
  )
}
