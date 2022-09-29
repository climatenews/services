import { NewsFeedUrlReferences } from 'graphql/generated/graphql'
import {timeSince } from 'app/time'



interface NewsItemContentProps {
    newsFeedUrlReferences: NewsFeedUrlReferences[]
}

export default function NewsItemContent(props: NewsItemContentProps) {

    return (
        <div className="container mx-auto ">
            <h3 className="text-2xl font-bold text-gray-900 text-left my-4">Shares</h3>
            <ul>
            {props.newsFeedUrlReferences && props.newsFeedUrlReferences.map((newsFeedUrl: NewsFeedUrlReferences, index: number) => {
                return (
                    <li key={newsFeedUrl.text}>
                        <p className="text-m font-bold">{newsFeedUrl.username}</p>
                        <p className="text-m">{newsFeedUrl.text}</p>
                    </li>
                )
            })}
        </ul>
        </div>
    )
}