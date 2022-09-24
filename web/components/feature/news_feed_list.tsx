import { NewsFeedUrl } from 'graphql/generated/graphql'

interface NewsFeedListProps {
    newsFeedUrls: NewsFeedUrl[]
}

export default function NewsFeedList(props: NewsFeedListProps) {    
    return (

        <ul>
            {props.newsFeedUrls && props.newsFeedUrls.map((newsFeedUrl: NewsFeedUrl) => {
                return (
                    <li key={newsFeedUrl.parsedExpandedUrl}>
                        <p className="text-lg"><a href={newsFeedUrl.parsedExpandedUrl} >{newsFeedUrl?.title}</a></p>
                        <p className="text-xs text-gray-400">{newsFeedUrl.numReferences} Shares, {new Date(newsFeedUrl.createdAt *1000).toUTCString()}</p>
                    </li>
                )
            })}
        </ul>
    )
}