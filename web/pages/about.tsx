import type { NextPage } from "next";
import AboutContent from "components/feature/about_content";
import Header from "components/generic/header";

const AboutPage: NextPage = () => {
  return (
    <>
      <Header />
      <AboutContent />
    </>
  );
};

export default AboutPage;
