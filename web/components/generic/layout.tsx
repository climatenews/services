import Footer from "components/generic/footer";
import NavBar from "components/generic/navbar";

export default function Layout({ children }) {
  return (
    <>
      <main>
        <div>{children}</div>
      </main>
      <Footer />
    </>
  );
}
