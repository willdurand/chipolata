import Chip8 from "./chip8";

Chip8.init(document, screen);

fetch("./space-invaders.ch8").then(async (response) => {
  const buffer = await response.arrayBuffer();
  const rom = new Uint8Array(buffer);

  Chip8.run(rom);
});
