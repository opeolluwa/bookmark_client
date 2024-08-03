import AppNavigation from "@/components/Navigation/AppNavigation";


interface Props {
  children: React.ReactNode;
}

export default function Layout({ children }: Props) {
  return (
    <div
      className="sm:grid grid-cols-12 mb-0 pb-0 w-[100vw]"
      id="layout"
      style={{
        height: "100vh",
        overflowY: "hidden",
        marginBottom: 0,
      }}
    >
      <AppNavigation />
      <main className="col-span-9 lg:col-span-9 pt-10 px-10 bg-[rgba(241,246,251,255)]  overflow-y-scroll">
        {children}
      </main>
    </div>
  );
}
