import { NewsFeedUrl } from 'graphql/generated/graphql'
import {timeSince } from 'app/time'
import Link from 'next/link'

interface NewsFeedListProps {
    newsFeedUrls: NewsFeedUrl[]
}

export default function NewsFeedList(props: NewsFeedListProps) {    
    return (

        <ul>
            {props.newsFeedUrls && props.newsFeedUrls.map((newsFeedUrl: NewsFeedUrl, index: number) => {
                return (
                    <li key={newsFeedUrl.expandedUrlParsed}>
                        <div className='flex items-baseline'>
                            <p className="text-lg mr-1">
                                {`${index+1}. `}
                                <a className="hover:underline" href={newsFeedUrl.expandedUrlParsed}>{newsFeedUrl?.title}</a>
                            </p> 
                            <p className="text-xs text-gray-400">({newsFeedUrl.expandedUrlHost})</p> 
                        </div>
                        <p className="text-s text-gray-400 ml-5">
                        <Link 
                            href={{
                                pathname: '/news_item/[item_id]',
                                query: { item_id: newsFeedUrl.urlId },
                            }}
                            >
                            <a className="hover:underline">
                                {newsFeedUrl.numReferences} Shares
                            </a>
                        </Link>
                            {` | ${timeSince(new Date(newsFeedUrl.createdAt *1000))}`}
                        </p>

                    </li>
                )
            })}
        </ul>
    )
}