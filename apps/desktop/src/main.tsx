import { Toaster } from '@components/ui/sonner'
import React from 'react'
import ReactDOM from 'react-dom/client'
import { createBrowserRouter, RouterProvider } from 'react-router-dom'

import App from './App'
import { Card } from './pages/card'
import { CreateCard } from './pages/create-card'
import { Deck } from './pages/deck'
// import { Error } from './pages/error'
import { Quiz } from './pages/quiz'
import { ReviewQuiz } from './pages/quiz/review'
import { Stats } from "./pages/stats"

const router = createBrowserRouter([
  {
    path: '/',
    element: <App />,
    // errorElement: <Error />
  },
  {
    path: '/stats',
    element: <Stats />,
    // errorElement: <Error />
  },
  {
    path: 'decks/:deckId',
    element: <Deck />,
    // errorElement: <Error />
  },
  {
    path: 'decks/:deckId/cards/:cardId',
    element: <Card />,
    // errorElement: <Error />
  },
  {
    path: 'createCard/:deckId',
    element: <CreateCard />,
    // errorElement: <Error />
  },
  {
    path: 'quiz/:deckId/',
    element: <Quiz />,
    // errorElement: <Error />
  },
  {
    path: 'quiz/:deckId/review',
    element: <ReviewQuiz />,
    // errorElement: <Error />
  },
])


ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
    <Toaster richColors closeButton position="bottom-left" duration={1400} />
  </React.StrictMode>,
)
