import { Button } from '@components/ui/button'
import { FadersHorizontal } from 'phosphor-react'
import { Link } from 'react-router-dom'

import {
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '../ui/dialog'
import { useEffect, useState } from "react"
import { invoke } from "@tauri-apps/api/core"

interface DeckCardProps {
  data: {
    title: string
    id: string
  }
}

export const DeckDialog = ({ data }: DeckCardProps) => {
  const [correctPercentage, setCorrectPercentage] = useState(0)
  const [totalFlashcards, setTotalFlashcards] = useState(0)
  const [flashcards_left, setFlashcards_left] = useState(0)

  async function get_correct_percentage() {
    const res = await invoke<number>('get_correct_percentage', { deckId: data.id })
    setCorrectPercentage(res)
  }
  
  async function get_total_flashcards_by_deck() {
    const res = await invoke<number>('get_total_flashcards_by_deck', { deckId: data.id })
    setTotalFlashcards(res)
  }

  async function find_flashcards_left_for_review() {
    const res = await invoke<number>('find_flashcards_left_for_review', { deckId: data.id })
    setFlashcards_left(res)
  }

  useEffect(() => {
    get_correct_percentage()
    get_total_flashcards_by_deck()
    find_flashcards_left_for_review()
  }, [])

  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>{data.title}</DialogTitle>
        <DialogDescription>
          Comece seus estudos aqui. Clique em editar para mais detalhes
        </DialogDescription>
      </DialogHeader>

      <div className="flex flex-col">
        <div className="flex">
          <span className="text-muted-foreground">Total de cards:&nbsp;</span>
          <span className="text-primary">{totalFlashcards}</span>
        </div>
        <div className="flex">
          <span className="text-muted-foreground">
            Cards faltando hoje:&nbsp;
          </span>
          <span className="text-primary">{flashcards_left}</span>
        </div>
        <div className="flex">
          <span className="text-muted-foreground">Acertos:&nbsp;</span>
          <span className="text-primary">{correctPercentage}%</span>
        </div>
      </div>

      <div className="mt-4 flex justify-between">
        <Link to={`/quiz/${data.id}`}>
          <Button>Começar !</Button>
        </Link>
        <Link to={`/decks/${data.id}`}>
          <Button variant="secondary">
            <FadersHorizontal className="mr-2 h-4 w-4" weight="bold" />
            Editar
          </Button>
        </Link>
      </div>
    </DialogContent>
  )
}
