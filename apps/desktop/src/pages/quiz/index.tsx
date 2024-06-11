import { Button } from '@components/ui/button'
import { invoke } from "@tauri-apps/api/core"
import { SignOut } from 'phosphor-react'
import { useEffect, useState } from 'react'
import { Link, useNavigate, useParams } from 'react-router-dom'

interface Flashcard {
  front: string
}

export const Quiz = () => {
  const { deckId } = useParams()
  const [flashcard, setFlashcard] = useState<Flashcard>({} as Flashcard)

  const navigation = useNavigate()

  function reviewQuiz() {
    navigation(`/quiz/${deckId}/review`, {
      state: { flashcard },
    })
  }

  useEffect(() => {
    invoke<Flashcard>('find_flashcard_for_review', { deckId }).then(card => setFlashcard(card))
    console.log(flashcard)
  }, [])

  return (
    <div className="flex h-screen flex-col p-6">
      <div className="flex justify-end">
        <Link to="/">
          <Button variant="destructive">
            <SignOut size={18} weight="bold" className="mr-2" />
            Sair
          </Button>
        </Link>
      </div>

      <div className="my-3 flex flex-1 items-center justify-center rounded-md bg-muted">
          {Object.keys(flashcard).length === 0 && (
            <p className="text-xl">Você já completou todos os flashcards de hoje!</p>
          )}
        <p className="text-3xl">
          {flashcard.front}
        </p>
      </div>

      {Object.keys(flashcard).length !== 0 && <Button onClick={reviewQuiz}>Revelar</Button>}
    </div>
  )
}
