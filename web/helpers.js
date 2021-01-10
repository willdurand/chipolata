const makeOscillator = (audioContext) => {
  const oscillator = audioContext.createOscillator();

  oscillator.type = "sine";
  oscillator.frequency.value = 400;
  oscillator.connect(audioContext.destination);

  return oscillator;
};

export const createAudio = () => {
  if (!window.AudioContext && !window.webkitAudioContext) {
    return null;
  }

  const audioContext = new (window.AudioContext || window.webkitAudioContext)();

  return {
    oscillator: null,

    start() {
      if (!this.oscillator) {
        this.oscillator = makeOscillator(audioContext);
        this.oscillator.start();
      }
    },

    stop() {
      if (this.oscillator) {
        this.oscillator.stop();
        this.oscillator = null;
      }
    },
  };
};

const makePixel = (context, color) => {
  const pixel = context.createImageData(1, 1);

  pixel.data[0] = color;
  pixel.data[1] = color;
  pixel.data[2] = color;
  pixel.data[3] = 255;

  return pixel;
};

export const createScreen = ($canvas, width, height) => {
  $canvas.width = width;
  $canvas.height = height;

  const screen = $canvas.getContext("2d");

  const BLACK_PIXEL = makePixel(screen, 0);
  const WHITE_PIXEL = makePixel(screen, 255);

  return {
    drawPixelAt(white, x, y) {
      screen.putImageData(white ? WHITE_PIXEL : BLACK_PIXEL, x, y);
    },
  };
};

export const makeKeypad = (keysPressed) => {
  return [
    !!keysPressed["x"], // 0
    !!keysPressed["1"], // 1
    !!keysPressed["2"], // 2
    !!keysPressed["3"], // 3
    !!keysPressed["q"], // 4
    !!keysPressed["w"], // 5
    !!keysPressed["e"], // 6
    !!keysPressed["a"], // 7
    !!keysPressed["s"], // 8
    !!keysPressed["d"], // 9
    !!keysPressed["z"], // A
    !!keysPressed["c"], // B
    !!keysPressed["4"], // C
    !!keysPressed["r"], // D
    !!keysPressed["f"], // E
    !!keysPressed["v"], // F
  ];
};
