"use client";

import { AppLogo } from "@/components/App/AppLogo";
import Button from "@/components/Button";
import Card from "@/components/Card";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import { LockClosedIcon, UserIcon } from "@heroicons/react/24/solid";
import { FormEvent, useState } from "react";

export default function Login() {
  const [active2Fa, use2Fa] = useState(false);
  const [accountExists, setCurrenAccount] = useState(true);

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

  if (accountExists) {
    return (
      <div
        id="login"
        className="flex flex-col items-center justify-center w-screen h-screen bg-gray-500 bg-[url(/assets/pattern.webp)] bg-blend-multiply"
      >
        <Card className="flex flex-col gap-6 pt-12 items-center justify-center bg-white shadow-lg shadow-black">
          <AppLogo className="hidden" />
          <Heading className=""> Welcome back, Adeoye!</Heading>
          <form onSubmit={onSubmit} className="px-12 my-8">
            <div className="flex gap-x-2 mb-2 items-end justify-start">
              <LockClosedIcon className="w-6 h-6 text-gray-500" />
              <input
                autoFocus
                type="password"
                className=" px-0 focus:border-app-500 focus:outline-none border-b-2 bg-white border-gray-500 block placeholder:pb-2 py-[2px]"
                placeholder="password"
              />
            </div>

            <Button
              className="bg-app-secondary-400 rounded-full text-white text-center w-full mt-6"
              href="/dashboard"
            >
              {" "}
              Login
            </Button>
          </form>

          <div className="flex justify-end gap-x-4">
            <SmallText
              onClick={() => setCurrenAccount(false)}
              className="text-gray-300 underline cursor-default"
            >
              Not Adeoye?
            </SmallText>
          </div>
        </Card>
      </div>
    );
  } else {
    return (
      <div
        id="login"
        className="flex flex-col items-center justify-center w-screen h-screen bg-gray-500 bg-[url(/assets/pattern.webp)] bg-blend-multiply"
      >
        <Card className="flex flex-col gap-6 pt-12 items-center justify-center bg-white shadow-lg shadow-black">
          <Heading>Welcome back!</Heading>
          <form
            onSubmit={onSubmit}
            className="px-12 my-8 flex flex-col gap-y-4"
          >
            <div className="flex gap-x-2 mb-2 items-end justify-start">
              <UserIcon className="w-6 h-6 text-gray-500" />
              <input
                autoFocus
                type="email"
                className=" px-0 focus:border-app-500 focus:outline-none border-b-2 bg-white border-gray-500 block placeholder:pb-2 py-[2px]"
                placeholder="email"
              />
            </div>
            <div className="flex gap-x-2 mb-2 items-end justify-start">
              <LockClosedIcon className="w-6 h-6 text-gray-500" />
              <input
                autoFocus
                type="password"
                className=" px-0 focus:border-app-500 focus:outline-none border-b-2 bg-white border-gray-500 block placeholder:pb-2 py-[2px]"
                placeholder="password"
              />
            </div>

            <Button
              className="bg-app-secondary-400 rounded-full text-white text-center w-full mt-6"
              href="/dashboard"
            >
              {" "}
              Login
            </Button>
          </form>
        </Card>
      </div>
    );
  }
}
