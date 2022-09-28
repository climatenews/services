import type { NextPage } from 'next'
import Footer from 'components/generic/footer'
import NewsContent from 'components/feature/news_content'
import { NewsFeedUrl, getSdk } from 'graphql/generated/graphql'
import { graphQLClient } from 'graphql/client';
import { useRouter } from 'next/router'

interface NewsPageProps {
  newsFeedUrls: NewsFeedUrl[]
}

const NewsItemPage: NextPage<NewsPageProps> = ({ newsFeedUrls }) => {
  const router = useRouter()
  const { item_id } = router.query
  return (
    <>
      <p>{item_id}</p>
      <Footer />
    </>
  )
}

export async function getServerSideProps(context: any) {
  const sdk = getSdk(graphQLClient)
  const response = await sdk.GetNewsFeedUrls()
  return {
    props: {
      newsFeedUrls: response.newsFeedUrls
    },
  }
}


export default NewsItemPage
