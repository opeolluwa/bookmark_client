"use client";

import { AppLogo } from "@/components/App/inedx";
import Card from "@/components/Card";
import Text from "@/components/Text";
import { FormEvent } from "react";

export default function Login() {
  async function onSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const formData = new FormData(event.currentTarget);
   

    // Handle response if necessary
    console.log({
      data: JSON.stringify(formData)
    })
    // ...
  }
  async function fetchSecurityQuestion(){

  }
  return (
    <div
      id="login"
      className="flex flex-col items-center justify-center w-screen h-screen bg-gray-100"
    >
      <AppLogo className="" />
      <Card className=" bg-white shadow-lg bg-re-500 border-2 border-gray-100  py-24 px-24 mt-6 ">
        {/* <Heading> Welcome back!</Heading> */}
        <Text className="mb-3 text-center">
          What&apos;s you mother&apos;s maiden name?
        </Text>
        <form onSubmit={onSubmit}>
          <input
            type="password"
            className="py-3 px-4 rounded-md focus:border-app-300 focus:outline-none border-2 bg-gray-100 boder-gray-400 text-center "
            autoFocus
          />
        </form>
      </Card>
    </div>
  );
}
