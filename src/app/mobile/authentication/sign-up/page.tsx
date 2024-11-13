"use client";
import { FormFieldTypes } from "@/app/page";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import { ArrowLongLeftIcon } from "@heroicons/react/24/solid";
import { invoke } from "@tauri-apps/api/core";
import { Form, FormProps, Input } from "antd";
import { useRouter } from "next/navigation";
import { LoginData } from "../../../../../tauri/bindings/LoginData";
import Link from "next/link";
import SmallText from "@/components/SmallText";

export default function LoginWithEmail() {
  const [form] = Form.useForm();
  const router = useRouter();
  const submit_form: FormProps<FormFieldTypes>["onFinish"] = (values) => {
    const new_user: LoginData = {
      email: values.email?.trim(),
      password: values.password?.trim(),
    };
    invoke("sign_in", { user: new_user }).then((res) => {
      console.log({ res });
    });
  };
  return (
    <>
      <Form
        initialValues={{ remember: true }}
        onFinish={submit_form}
        autoComplete="off"
        name="save-data"
        layout="vertical"
        className=" flex flex-col rounded-lg shadow-gray-300 gap-y-2 py-6"
        form={form}
      >
        <View className="mb-14 flex justify-end items-center">
          <Link href="/mobile/" className="text-app font-bold">
            Have an account? Log in
          </Link>
        </View>

        <View className="mb-12">
          <Heading className="font-bold">Create your account</Heading>
          <Text className="leading-1 mt-2 text-sm">
            Let&apos;s create an account for free to get started
          </Text>
        </View>

        <Form.Item<FormFieldTypes> label="Email" name="email">
          <Input
            type="email"
            name="email"
            className="w-full rounded-lg py-4 focus:border-app-500 focus:outline-none border-[2px] bg-white border-gray-300 placeholder:pb-2 px-2 "
            placeholder="enter your email"
          />
        </Form.Item>

        <button
          onClick={() => router.push("/dashboard")}
          className="w-full rounded-lg py-4 bg-app-600 font-medium text-white "
        >
          Continue
        </button>
      </Form>
      <SmallText className=" text-sm text-gray-400">
        By clicking continue you agree to the terms of{" "}
        <a href="#">terms of service </a> and <a href="#">privacy policy</a>
      </SmallText>
    </>
  );
}
