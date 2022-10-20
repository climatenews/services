import Footer from "components/generic/footer";

type LayoutProps = React.PropsWithChildren<{}>;

export default function Layout({ children }: LayoutProps) {
  return (
    <>
      <main className="bg-gray-50 font-sans leading-normal tracking-normal">
        <div>{children}</div>
      </main>
      <Footer />
    </>
  );
}
