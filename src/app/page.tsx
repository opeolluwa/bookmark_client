"use client";
import { OsType, type } from "@tauri-apps/plugin-os";
import { useEffect, useState } from "react";
import DesktopAppEntry from "./desktop";
import MobileAppEntry from "./mobile";

export type FormFieldTypes = {
  email: string;
  password: string;
};

export default function LoginPage() {
  const [osType, setOsType] = useState<OsType>("" as OsType);
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    const fetchData = () => {
      const os = type();
      setOsType(os);

      if (os == "android" || os == "ios") {
        setIsMobile(true);
      }
    };

    fetchData();
  }, []);

  if (isMobile) {
    return <MobileAppEntry />;
  } else {
    return <DesktopAppEntry />;
  }
}
