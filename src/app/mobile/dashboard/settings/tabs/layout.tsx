"use client";
import View from "@/components/View";
import { ArrowLongLeftIcon } from "@heroicons/react/24/outline";
import { Divider } from "antd";
import { useRouter } from "next/navigation";
import { ReactNode } from "react";

export default function Page({ children }: { children: ReactNode }) {
  const router = useRouter();
  return (
    <>
      <View className="flex items-center justify-between">
        <ArrowLongLeftIcon className="size-5" onClick={() => router.back()} />
      </View>
      <Divider />

      {children}
    </>
  );
}
