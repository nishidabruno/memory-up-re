import { ChartPieSlice, SquaresFour } from "phosphor-react"
import { useLocation, useNavigate } from "react-router-dom"

export const Sidebar = () => {
  const navigate = useNavigate()
  const { pathname } = useLocation()

  return (
    <div className="w-14 flex flex-col bg-muted px-3 py-5 gap-4 h-screen fixed">
      <a className="cursor-pointer" onClick={() => navigate('/')}>
        <SquaresFour size={32} className={`${'/' === pathname ? 'text-zinc-200' : 'text-zinc-500'} hover:text-zinc-200 transition-colors`} />
      </a>
      <a className="cursor-pointer" onClick={() => navigate('/stats')}>
        <ChartPieSlice size={32} className={`${'/stats' === pathname ? 'text-zinc-200' : 'text-zinc-500'} hover:text-zinc-200 transition-colors`} />
      </a>
    </div>
  )
}