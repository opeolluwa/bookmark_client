"use client";
import { FormFieldTypes } from "@/app/page";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import { invoke } from "@tauri-apps/api/core";
import { Form, FormProps, Input } from "antd";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { LoginData } from "../../../../../tauri/bindings/LoginData";
import { ArrowLongLeftIcon } from "@heroicons/react/24/solid";

export default function ForgottenPassword() {
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
        className=" flex flex-col rounded-lg shadow-gray-300 pb-6"
        form={form}
      >
        <View className="mb-12 flex justify-between items-center">
          <Link href="/mobile/">
            <ArrowLongLeftIcon className="w-6 h-6" />
          </Link>
        </View>

        <View className="mb-6">
          <Heading className="font-bold">Let&apos;s get you back in</Heading>
          <Text className="leading-1 mt-2 text-sm">
            Provide the email associated with your account and we&apos; send
            more instruction
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
          onClick={() => router.push("/mobile/authentication/password-otp")}
          className="w-full rounded-lg py-4 bg-app-600 font-medium text-white "
        >
          Continue
        </button>
      </Form>
      <Link
        href="/mobile"
        className="text-app text-sm font-semibold"
      >
        Remember the password? Log in
      </Link>
    </>
  );
}
