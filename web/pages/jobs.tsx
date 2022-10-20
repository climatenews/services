import type { NextPage } from "next";
import JobsContent from "components/feature/jobs_content";
import Meta from "components/generic/meta";
import NavBar from "components/generic/navbar";

const JobsPage: NextPage = () => {
  return (
    <>
      <Meta />
      <NavBar pageRoute="/jobs" />
      <JobsContent />
    </>
  );
};

export default JobsPage;
