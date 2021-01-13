export const hexformat = (val, size) => {
  return `0x${val.toString(16).padStart(size, "0")}`;
};
