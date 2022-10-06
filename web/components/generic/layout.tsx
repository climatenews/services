import Footer from "components/generic/footer";

export default function Layout({ children }) {
  return (
    <>
      <main>
        <div className="container mx-auto p-4">{children}</div>
      </main>
      <Footer />
    </>
  );
}
