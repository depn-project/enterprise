export const ipv4 = () => {
  const test = (ip: string) =>
    !!/^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/.test(
      ip
    );

  return async (value: string) => {
    return { valid: test(value), name: "invalid_ip_address" };
  };
};

export const port = () => {
  const test = (port: string) =>
    !!/^((6553[0-5])|(655[0-2][0-9])|(65[0-4][0-9]{2})|(6[0-4][0-9]{3})|([1-5][0-9]{4})|([0-5]{0,5})|([0-9]{1,4}))$/gi.test(
      port
    );

  return async (value: string) => {
    return { valid: test(value), name: "invalid_port" };
  };
};
