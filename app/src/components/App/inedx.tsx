import Heading from "../Heading";
import View from "../View";
import Image from "next/image";
import CarbonylLogo from "/assets/carbonyl.png";

interface Props {
    className?: string
}
export function AppLogo({className}:Props){
    return (
      <>
        <View className={"flex items-center justify-left "+  className}>
          <img
            src="/assets/carbonyl.png"
            alt={"Carbonyl Logo"}
            className="w-[45px]"
          ></img>{" "}
          <Heading>
            Carbonyl<span className="text-app font-bold">.</span>
          </Heading>
        </View>
      </>
    );
}