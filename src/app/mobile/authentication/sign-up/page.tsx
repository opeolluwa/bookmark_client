"use client";
import { httpClient } from "@/app/axios";
import SubmitButton from "@/components/Button/SubmitButton";
import Heading from "@/components/Heading";
import SmallText from "@/components/SmallText";
import Text from "@/components/Text";
import View from "@/components/View";
import { useUserProfileInformation } from "@/context/userInformation";
import { Form, FormProps, Input } from "antd";
import type { SignUpRequest } from "bookmark_grpc_codegen/bindings/SignUpRequest";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState } from "react";

type FormFieldTypes = SignUpRequest;

export default function LoginWithEmail() {
  const [form] = Form.useForm();
  const router = useRouter();
  const [isProcessingForm, setIsProcessingForm] = useState(false);
  const submitForm: FormProps<FormFieldTypes>["onFinish"] = async (values) => {
    const signUpRequest: SignUpRequest = {
      email: values.email?.trim(),
      password: values.password?.trim(),
      first_name: values.first_name.trim(),
      last_name: values.last_name.trim(),
    };

    setIsProcessingForm(true);
    await httpClient
      .post("/users/register", {
        ...signUpRequest,
      })
      .then((response) => {
        setIsProcessingForm(false);
        // if(response.status!==200){}
        console.log(response);
        router.push("/mobile/dashboard");
      });
  };
  return (
    <>
      <Form
        initialValues={{ remember: true }}
        onFinish={submitForm}
        autoComplete="off"
        name="save-data"
        layout="vertical"
        className=" flex flex-col rounded-lg shadow-gray-300 pb-6"
        form={form}
      >
        <View className="mb-12 flex justify-end items-center">
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

        <Form.Item<FormFieldTypes> label="First name" name="first_name">
          <Input
            type="firstname"
            name="firstname"
            className="w-full rounded-lg py-4 focus:border-app-500 focus:outline-none border-[2px] bg-white border-gray-300 placeholder:pb-2 px-2 "
            placeholder="enter your first name"
          />
        </Form.Item>

        <Form.Item<FormFieldTypes> label="Last name" name="last_name">
          <Input
            type="text"
            name="lastname"
            className="w-full rounded-lg py-4 focus:border-app-500 focus:outline-none border-[2px] bg-white border-gray-300 placeholder:pb-2 px-2 "
            placeholder="enter your last name"
          />
        </Form.Item>

        <Form.Item<FormFieldTypes> label="Email" name="email">
          <Input
            type="email"
            name="email"
            className="w-full rounded-lg py-4 focus:border-app-500 focus:outline-none border-[2px] bg-white border-gray-300 placeholder:pb-2 px-2 "
            placeholder="enter your email"
          />
        </Form.Item>

        <Form.Item<FormFieldTypes> label="Password" name="password">
          <Input
            type="password"
            name="password"
            className="w-full rounded-lg py-4 focus:border-app-500 focus:outline-none border-[2px] bg-white border-gray-300 placeholder:pb-2 px-2 "
            placeholder="choose a password"
          />
        </Form.Item>
        <SubmitButton
          className="w-full rounded-lg py-4 bg-app-600 font-medium text-white"
          text="Continue"
          loadingState={isProcessingForm}
        />
      </Form>
      <SmallText className=" text-sm text-gray-400">
        By clicking continue you agree to the terms of{" "}
        <a href="#">terms of service </a> and <a href="#">privacy policy</a>
      </SmallText>
    </>
  );
}
