import { isRouteErrorResponse, useRouteError } from 'react-router-dom'

export const Error = () => {
  const error = useRouteError()

  if (isRouteErrorResponse(error)) {
    return (
      <div>
        <h1 color="white">Oops, algo está errado!</h1>
        <h2>Tente reabrir o programa</h2>
        <p>{error.statusText}</p>
      </div>
    )
  }

  return <p>Algo deu errado, tente reabrir o programa.</p>
}
