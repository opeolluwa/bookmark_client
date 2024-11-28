// import { userInformationStore } from "@/stores/userInformation";
import { message } from "@tauri-apps/plugin-dialog";
import { AxiosError } from "axios";
import { LoginRequest } from "bookmark_grpc_codegen/bindings/LoginRequest";
import { httpClient } from "../app/axios";

export async function loginHandler(payload: LoginRequest) {
  try {
    const response = await httpClient.post("/users/login", { ...payload });
    console.log(response);
    // await userInformationStore.set("userInformation", { ...response.data });
    return;
  } catch (error) {
    if (error instanceof AxiosError) {
      await message(
        error.response?.data?.message || "Failed to login. Please try again.",
        { kind: "error", title: "Login error " }
      );
    }
    await message("Failed to login. Please try again.", {
      kind: "error",
      title: "Login error ",
    });
  }
}
