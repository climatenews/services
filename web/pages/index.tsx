import type { NextPage } from "next";
import Footer from "components/generic/footer";
import AboutContent from "components/feature/about_content";

const Home: NextPage = () => {
  return (
    <>
      <AboutContent />
      <Footer />
    </>
  );
};

export default Home;
