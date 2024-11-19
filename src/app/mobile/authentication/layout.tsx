"use client";

import View from "@/components/View";

export default function LoginWithEmail({
  children,
}: {
  children: React.ReactNode;
}) {
  return <View className="py-14 px-6 relative h-auto ">{children}</View>;
}