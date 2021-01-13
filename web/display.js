export const createDisplay = (
  $canvas,
  width,
  height,
  screenWidth,
  screenHeight
) => {
  // Original dimensions.
  $canvas.width = width;
  $canvas.height = height;

  // Actual dimensions on the page. We substract `40` so that the display
  // becomes smaller than the actual device screen on small screens.
  const displayWidth = Math.min(screenWidth - 40, width * 10);
  $canvas.style.width = `${displayWidth}px`;
  $canvas.style.height = `${displayWidth / 2}px`;

  const display = $canvas.getContext("2d");

  return {
    draw(vram) {
      const imageData = display.createImageData(width, height);

      for (let i = 0; i < vram.length; i++) {
        imageData.data[i * 4] = vram[i] === 1 ? 255 : 0;
        imageData.data[i * 4 + 1] = vram[i] === 1 ? 255 : 0;
        imageData.data[i * 4 + 2] = vram[i] === 1 ? 255 : 0;
        imageData.data[i * 4 + 3] = 255;
      }

      display.putImageData(imageData, 0, 0);
    },
  };
};
