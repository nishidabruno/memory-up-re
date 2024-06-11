import { Separator } from '@components/ui/separator'
import { convertToFurigana } from '@lib/convert-to-furigana'

interface CardPreviewProps {
  data: {
    front: string
    back: string
  }
}

export const CardPreview = ({ data }: CardPreviewProps) => {
  const furigana = convertToFurigana(data?.back)

  return (
    <div className="flex w-full flex-col items-center justify-center rounded-sm bg-muted p-9">
      <p className="text-xl text-muted-foreground">{data.front}</p>
      <Separator orientation="horizontal" className="my-4 bg-gray-700" />
      <p
        className="text-xl text-muted-foreground"
        dangerouslySetInnerHTML={{ __html: furigana }}
      />
    </div>
  )
}
