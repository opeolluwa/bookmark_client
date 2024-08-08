import SearchBar from "@/components/Search";
import View from "@/components/View";
import AppLayout from "@/Layouts/AppLayout";
import { BellAlertIcon, BellIcon, MoonIcon } from "@heroicons/react/24/outline";

export default function Home() {
  return (
    <AppLayout>
        <View className=" h-screen ">
          <View className="flex justify-between items-center">
            {/* <Heading>Hello Opeoluwa!</Heading>
            <Text className="mt-2">
              Track team progress here do whatever you like{" "}
            </Text> */}
            <View className="w-4/5">
              <SearchBar
                onSearch={console.log("hey")}
                placeholder={""}
                className="py-2 shadow-none rounded-full w-full "
              />
            </View>
            <View className="flex items-center justify-center gap-4">
              <BellIcon className="w-8 h-8 p-2 bg-slate-100 hover:bg-app-300 cursor-pointer  rounded-full" />
              <MoonIcon className="w-8 h-8 p-2 bg-slate-100 hover:bg-app-300 cursor-pointer  rounded-full" />
              <img
                src="/assets/default-avatar.png"
                alt=" avatar"
                className="w-[36px] cursor-pointer"
              />
            </View>
          </View>
          <View className=" px-4 bg-slate-50  mt-8 py-8 rounded-xl h-screen shadow">

          </View>
      </View>
      
    </AppLayout>
  );
}
