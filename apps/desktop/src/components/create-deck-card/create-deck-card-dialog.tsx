import { Button } from '@components/ui/button'
import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@components/ui/dialog'
import { Input } from '@components/ui/input'
import { invoke } from "@tauri-apps/api/core"
import { useState } from 'react'
import { toast } from 'sonner'

export function CreateDeckCardDialog() {
  const [deckTitle, setDeckTitle] = useState('')

  async function createDeck() {
    try {
      await invoke('create_deck', { title: deckTitle })
    } catch (err) {
      toast.error(
        `Error ao criar deck novo. Tente novamente mais tarde. ${err}`)
    }

    return toast.success('Deck criado com sucesso!', { duration: 1500, position: 'bottom-left' })
  }

  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>Criar novo deck</DialogTitle>
        <DialogDescription>Digite o nome do novo deck</DialogDescription>
      </DialogHeader>

      <Input
        placeholder="Ex: Kanji"
        className="w-full"
        onChange={(e) => setDeckTitle(e.target.value)}
      />
      <DialogClose asChild>
        <Button onClick={() => createDeck()}>Criar</Button>
      </DialogClose>
    </DialogContent>
  )
}
