import Footer from "components/generic/footer";

export default function Layout({ children }) {
  return (
    <>
      {/* <Navbar /> */}
      <main>
        <div className="container mx-auto px-4">{children}</div>
      </main>
      <Footer />
    </>
  );
}
