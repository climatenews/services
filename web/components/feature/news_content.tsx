import { NewsFeedUrl } from 'graphql/generated/graphql'
import NewsFeedList from './news_feed_list';


interface NewsContentProps {
    newsFeedUrls: NewsFeedUrl[]
}

export default function NewsContent(props: NewsContentProps) {

    return (
        <div className="container mx-auto ">
            <h3 className="text-2xl font-bold text-gray-900 text-left my-4">Climate News</h3>
            <NewsFeedList newsFeedUrls={props.newsFeedUrls}/>
        </div>
    )
}