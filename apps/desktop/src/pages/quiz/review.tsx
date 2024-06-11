import { Button } from '@components/ui/button'
import { invoke } from "@tauri-apps/api/core"
import { SignOut } from 'phosphor-react'
import { Link, useLocation, useNavigate } from 'react-router-dom'

interface State {
  flashcard: {
    id: string
    deck_id: string
    back: string
  }
}

export const ReviewQuiz = () => {
  const navigation = useNavigate()
  const { flashcard } = useLocation().state as State

  async function review(quality: number) {
    await invoke('review_flashcard', {
      quality,
      flashcardId: flashcard.id,
      deckId: flashcard.deck_id
    })

    return navigation(`/quiz/${flashcard.deck_id}`)
  }

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
        <p className="text-2xl">{flashcard.back}</p>
      </div>

      <div className="flex justify-center gap-4">
        <Button onClick={() => review(0)} variant="outline">
          Não lembro
        </Button>
        <Button onClick={() => review(3)} variant="outline">
          Difícil
        </Button>
        <Button onClick={() => review(4)} variant="outline">
          Médio
        </Button>
        <Button onClick={() => review(5)} variant="outline">
          Fácil
        </Button>
      </div>
    </div>
  )
}
