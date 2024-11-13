"use client";
import { useState } from "react";
import AuthorizeWithPassword from "./authentication/sign-in/authorize-with-password";
import LoginWithEmail from "./authentication/sign-in/login-with-email-and-password";

export default function MobileAppEntry() {
  const [accountExist, setAccountExist] = useState(true);

  if (accountExist) {
    return <AuthorizeWithPassword />;
  } else {
    return <LoginWithEmail />;
  }
}
