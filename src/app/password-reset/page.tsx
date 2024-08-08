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
      <AppLogo className="mb-8" />
      <Card className="flex flex-col gap-6 items-center justify-center">
        <form onSubmit={onSubmit}>
          <Text className="mb-4">Please provide email to continue </Text>
          <input
            type="text"
            className="py-3 px-4 rounded-md focus:border-app-300 focus:outline-none border-2 bg-gray-100 boder-gray-400 block"
            placeholder="example@mailer.com"
          />
          <Button className="bg-app-400 text-gray-600 w-full mt-6">
            {" "}
            Continue
          </Button>
        </form>
      </Card>
    </div>
  );
}
