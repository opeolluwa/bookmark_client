import axios from "axios";

export const httpClient = axios.create({
  baseURL: "https://84da-102-88-37-101.ngrok-free.app/v1",
  timeout: 10000, // 5 seconds
});
