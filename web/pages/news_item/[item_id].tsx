import type { NextPage } from 'next'
import Footer from 'components/generic/footer'
import NewsItemContent from 'components/feature/news_item_content'
import { NewsFeedUrlReferences, getSdk } from 'graphql/generated/graphql'
import { graphQLClient } from 'graphql/client';
import { useRouter } from 'next/router'

interface NewsItemPageProps {
  newsFeedUrlReferences: NewsFeedUrlReferences[]
}

const NewsItemPage: NextPage<NewsItemPageProps> = ({ newsFeedUrlReferences }) => {

  return (
    <>
      <NewsItemContent newsFeedUrlReferences={newsFeedUrlReferences} />
      <Footer />
    </>
  )
}

export async function getServerSideProps(context: any) {
  const { item_id } = context.query
  const sdk = getSdk(graphQLClient)
  const response = await sdk.GetNewsFeedUrlReferences({urlId: Number(item_id)})
  return {
    props: {
      newsFeedUrlReferences: response.newsFeedUrlReferences
    },
  }
}


export default NewsItemPage
