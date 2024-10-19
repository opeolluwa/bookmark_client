"use client";

import { AppLogo } from "@/components/App/inedx";
import Button from "@/components/Button";
import Card from "@/components/Card";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import Text from "@/components/Text";
import { FormEvent, useState } from "react";

export default function Login() {
  const [active2Fa, use2Fa] = useState(false);

  async function onSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);

    // Handle response if necessary
    console.log({
      data: JSON.stringify(formData),
    });
    // ...
  }
  async function fetchSecurityQuestion() {}

  if (active2Fa) {
  }
  return (
    <div
      id="login"
      className="flex flex-col items-center justify-center w-screen h-screen bg-gray-100"
    >
      <AppLogo className="" />
      <Card className="flex flex-col gap-6 items-center justify-center">
        <form onSubmit={onSubmit}>
          <input
            type="text"
            className="py-3 px-4 rounded-md focus:border-app-300 focus:outline-none border-2 bg-gray-100 boder-gray-400 block my-6"
            autoFocus
            placeholder="username"
          />
          <input
            type="password"
            className="py-3 px-4 rounded-md focus:border-app-300 focus:outline-none border-2 bg-gray-100 boder-gray-400 block"
            placeholder="password"
          />
          <Button className="bg-app-400 text-gray-700 w-full mt-6">
            {" "}
            Login
          </Button>
        </form>
        <SmallText className="text-gray-300" href="/dashboard">
          Forgotten password?
        </SmallText>
      </Card>
    </div>
  );
}
