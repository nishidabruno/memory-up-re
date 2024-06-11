import { CreateDeckCard } from "@components/create-deck-card";
import { DeckCard } from "@components/deck-card";
import { ToggleTheme } from "@components/toggle-theme";
import { useEffect, useState } from "react";
import { Sidebar } from "@components/Sidebar";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@components/ui/button";

interface Deck {
  title: string;
  id: string;
}

export const Home = () => {
  const [decks, setDecks] = useState<Deck[]>([]);

  async function loadDecks() {
    const decks = await invoke<Deck[]>("find_all_decks");
    setDecks(decks);
  }

  useEffect(() => {
    loadDecks();
  }, []);

  return (
    <div className="flex h-full">
      <Sidebar />
      <div className="ml-14 flex-1 p-6 min-h-screen h-full">
        <div className="flex justify-between">
          <strong className="text-lg text-foreground">
            Olá, bom te ver denovo!
          </strong>
          <div className="flex gap-2">
            <CreateDeckCard />
            <ToggleTheme />
          </div>
        </div>

        <div className="mt-8 flex flex-wrap gap-4">
          {decks.map((deck) => (
            <DeckCard key={deck.id} data={deck} />
          ))}
        </div>
      </div>
    </div>
  );
};
