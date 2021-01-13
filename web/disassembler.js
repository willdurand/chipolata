import { hexformat } from "./utils";

const nnn = (opcode) => hexformat(opcode & 0x0fff, 4);
const x = (opcode) => (opcode & 0x0f00) >> 8;
const y = (opcode) => (opcode & 0x00f0) >> 4;
const kk = (opcode) => hexformat(opcode & 0x00ff, 2);
const nibble = (opcode) => opcode & 0x000f;

const disassembleAddr = (program, addr) => {
  const opcode = (program[addr] << 8) | program[addr + 1];

  // http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
  let instr = "-";
  switch (opcode & 0xf000) {
    case 0x0000:
      if (opcode == 0x00e0) {
        instr = "CLS";
      } else if (opcode == 0x00ee) {
        instr = "RET";
      }
      // TODO: support 0nnn maybe
      break;
    case 0x1000:
      instr = `JP ${nnn(opcode)}`;
      break;
    case 0x2000:
      instr = `CALL ${nnn(opcode)}`;
      break;
    case 0x3000:
      instr = `SE V${x(opcode)}, ${kk(opcode)}`;
      break;
    case 0x4000:
      instr = `SNE V${x(opcode)}, ${kk(opcode)}`;
      break;
    case 0x5000:
      instr = `SE V${x(opcode)}, V${y(opcode)}`;
      break;
    case 0x6000:
      instr = `LD V${x(opcode)}, ${kk(opcode)}`;
      break;
    case 0x7000:
      instr = `ADD V${x(opcode)}, ${kk(opcode)}`;
      break;
    case 0x8000:
      switch (nibble(opcode)) {
        case 0:
          instr = `LD V${x(opcode)}, V${y(opcode)}`;
          break;
        case 1:
          instr = `OR V${x(opcode)}, V${y(opcode)}`;
          break;
        case 2:
          instr = `AND V${x(opcode)}, V${y(opcode)}`;
          break;
        case 3:
          instr = `XOR V${x(opcode)}, V${y(opcode)}`;
          break;
        case 4:
          instr = `ADD V${x(opcode)}, V${y(opcode)}`;
          break;
        case 5:
          instr = `SUB V${x(opcode)}, V${y(opcode)}`;
          break;
        case 6:
          instr = `SHR V${x(opcode)} {, V${y(opcode)}}`;
          break;
        case 7:
          instr = `SUBN V${x(opcode)}, V${y(opcode)}`;
          break;
        case 0xe:
          instr = `SHL V${x(opcode)} {, V${y(opcode)}}`;
          break;
      }
      break;
    case 0x9000:
      instr = `SNE V${x(opcode)}, V${y(opcode)}`;
      break;
    case 0xa000:
      instr = `LD I, ${nnn(opcode)}`;
      break;
    case 0xb000:
      instr = `JP V0, ${nnn(opcode)}`;
      break;
    case 0xc000:
      instr = `RND V${x(opcode)}, ${kk(opcode)}`;
      break;
    case 0xd000:
      instr = `DRW V${x(opcode)}, ${kk(opcode)}, ${hexformat(
        nibble(opcode),
        1
      )}`;
      break;
    case 0xe000:
      if (kk(opcode) == 0x9e) {
        instr = `SKP V${x(opcode)}`;
      } else if (kk(opcode) == 0xa1) {
        instr = `SKNP V${x(opcode)}`;
      }
      break;
    case 0xf000:
      switch (opcode & 0x00ff) {
        case 0x07:
          instr = `LD V${x(opcode)}, DT`;
          break;
        case 0x0a:
          instr = `LD V${x(opcode)}, K`;
          break;
        case 0x15:
          instr = `LD DT, V${x(opcode)}`;
          break;
        case 0x18:
          instr = `LD ST, V${x(opcode)}`;
          break;
        case 0x1e:
          instr = `ADD I, V${x(opcode)}`;
          break;
        case 0x29:
          instr = `LD F, V${x(opcode)}`;
          break;
        case 0x33:
          instr = `LD B, V${x(opcode)}`;
          break;
        case 0x55:
          instr = `LD [I], V${x(opcode)}`;
          break;
        case 0x65:
          instr = `LD V${x(opcode)}, [I]`;
          break;
      }
  }

  return [instr];
};

export const disassemble = (ram) => {
  const lines = [];

  for (let addr = 0x201; addr < 0x1000; addr += 2) {
    const [instr] = disassembleAddr(ram, addr);
    lines.push(
      `<div class="addr-${addr}">${hexformat(addr, 4)}: ${instr}</div>`
    );
  }

  return lines.join("");
};
