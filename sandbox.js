import axios from "axios";

export const httpClient = axios.create({
  baseURL: "https://84da-102-88-37-101.ngrok-free.app/v1",
  timeout: 5000, // 5 seconds
});

await httpClient
  .post("/users/login", {
    first_name: "adeoye",
    last_name: "adefemi",
    email: "test3@mailer.com",
    password: "test",
  })
  .then((resp) => {
    console.log(resp.data);
  });
