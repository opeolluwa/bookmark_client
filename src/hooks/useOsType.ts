import { OsType, type } from "@tauri-apps/plugin-os";
import { useEffect, useState } from "react";

export function useIsMobile() {
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

  return isMobile;
}
