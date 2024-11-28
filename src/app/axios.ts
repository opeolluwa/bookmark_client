import axios from "axios";

export const httpClient = axios.create({
  baseURL: "https://ecfe-102-88-37-101.ngrok-free.app/v1",
  timeout: 5000,// 5 seconds 
});
