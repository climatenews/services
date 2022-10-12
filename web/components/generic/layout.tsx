import Footer from "components/generic/footer";

type LayoutProps = React.PropsWithChildren<{}>;

export default function Layout({ children } : LayoutProps) {
  return (
    <>
      <main>
        <div>{children}</div>
      </main>
      <Footer />
    </>
  );
}
