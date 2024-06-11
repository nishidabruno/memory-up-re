import { Button } from '@components/ui/button'
import { Dialog, DialogTrigger } from '@components/ui/dialog'
import { Plus } from 'lucide-react'

import { CreateDeckCardDialog } from './create-deck-card-dialog'

export function CreateDeckCard() {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline">
          <Plus className="mr-2 h-4 w-4" />
          Novo deck
        </Button>
      </DialogTrigger>

      <CreateDeckCardDialog />
    </Dialog>
  )
}
