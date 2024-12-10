import View from "../View";

interface Props {
  className?: string;
}
export function AppLogo({ className }: Props) {
  return (
    <>
      <View className={"flex items-center justify-left " + className}>
        <img
          src="/assets/vault.png"
          alt={"Vault Logo"}
          className="w-[45px]"
        ></img>
      </View>
    </>
  );
}
