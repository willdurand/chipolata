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
