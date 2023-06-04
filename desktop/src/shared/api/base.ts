import axios from "axios";

export const getBaseUrl = () => {
  const ip = localStorage.getItem("ip");
  const port = localStorage.getItem("port");

  return `${ip}:${port}/`;
};

export const apiInstance = axios.create({
  baseURL: getBaseUrl(),
});
