"use client";
import { useState } from "react";
import AuthorizeWithPassword from "./authentication/authorize";
import LoginWithEmail from "./authentication/login";

export default function MobileAppEntry() {

  const [accountExist, setAccountExist] = useState(true);

  if (accountExist) {
    return <AuthorizeWithPassword />;
  } else {
    return <LoginWithEmail />;
  }
}
