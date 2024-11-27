import axios from "axios";

export const httpClient = axios.create({
  baseURL: "http://localhost:4576/v1",
  timeout: 10000,
  //   headers:[],//TODO:
});
