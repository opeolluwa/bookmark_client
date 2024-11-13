"use client";

import View from "@/components/View";

export default function LoginWithEmail({
  children,
}: {
  children: React.ReactNode;
}) {
  return <View className="pt-14 px-6 relative min-h-screen ">{children}</View>;
}
